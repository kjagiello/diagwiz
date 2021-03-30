/// Cassowary based layout generator
use cassowary::strength::*;
use core::marker::PhantomData;

use ascii_canvas::{Draw, DrawResult, Rect, TextCanvas};
use cassowary::WeightedRelation::*;
use cassowary::{Constraint, Expression, Solver, Variable};
use std::sync::Arc;
use unicode_segmentation::UnicodeSegmentation;

/// Renderable layout node
/// A common set of variables for a renderable layout element
struct Vars<T> {
    /// X coordinate
    left: T,
    /// Y coordinate
    top: T,
    /// width
    width: T,
    /// height
    height: T,
}

// TODO: trait?
impl Vars<Variable> {
    /// Convenience method for creating new Vars
    fn new() -> Self {
        Self {
            left: Variable::new(),
            top: Variable::new(),
            width: Variable::new(),
            height: Variable::new(),
        }
    }

    fn right(&self) -> Expression {
        self.left + self.width
    }

    fn bottom(&self) -> Expression {
        self.top + self.height
    }

    fn center(&self) -> Expression {
        self.left + self.width / 2.0
    }
}

impl Vars<usize> {
    fn right(&self) -> usize {
        self.left + self.width
    }

    fn bottom(&self) -> usize {
        self.top + self.height
    }

    fn center(&self) -> usize {
        self.left + self.width / 2
    }
}

struct Node<T: Render<C>, C> {
    vars: Vars<Variable>,
    constraints: Vec<Constraint>,
    data: T,
    _phantom: PhantomData<C>,
}

impl<T: Render<C>, C> Node<T, C> {
    fn coords(&self, solver: &Solver) -> Vars<usize> {
        Vars {
            left: solver.get_value(self.vars.left) as usize,
            top: solver.get_value(self.vars.top) as usize,
            width: solver.get_value(self.vars.width) as usize,
            height: solver.get_value(self.vars.height) as usize,
        }
    }

    fn render<D: Draw>(&self, canvas: &mut D, ctx: &C) -> DrawResult {
        self.data.render(canvas, &ctx)
    }
}

trait Render<C> {
    fn render<D: Draw>(&self, canvas: &mut D, ctx: &C) -> DrawResult;

    fn width(&self, _ctx: &C) -> Option<usize> {
        None
    }

    fn height(&self, _ctx: &C) -> Option<usize> {
        None
    }
}

#[derive(PartialEq)]
pub struct Participant {
    pub id: String,
    pub name: String,
}

struct BareRenderCtx;

impl Render<BareRenderCtx> for Arc<Participant> {
    fn width(&self, _ctx: &BareRenderCtx) -> Option<usize> {
        Some(self.name.graphemes(true).count() + 4)
    }

    fn height(&self, _ctx: &BareRenderCtx) -> Option<usize> {
        Some(3)
    }

    fn render<D: Draw>(&self, canvas: &mut D, _ctx: &BareRenderCtx) -> DrawResult {
        let Rect { width, height, .. } = canvas.bounds();
        let name_len = self.name.graphemes(true).count();
        let mut repr = String::from("");
        let segments = [
            "┌",
            &"─".repeat(width - 2),
            "┐",
            "\n",
            &"│".repeat(height - 2),
            &" ".repeat(width - name_len - 3),
            &self.name,
            &" ".repeat(width - name_len - 3),
            &"│".repeat(height - 2),
            "\n",
            "└",
            &"─".repeat(width - 2),
            "┘",
        ];
        for segment in segments.iter() {
            repr.push_str(segment);
        }

        canvas.draw(
            0,
            0,
            &repr.split('\n').map(|s| s.to_string()).collect::<Vec<_>>(),
        )?;
        Ok(())
    }
}

pub struct Message {
    pub source: Arc<Participant>,
    pub target: Arc<Participant>,
    pub payload: String,
}

struct MessageRenderCtx {
    source_idx: usize,
    target_idx: usize,
}

impl MessageRenderCtx {
    fn is_loop(&self) -> bool {
        self.source_idx == self.target_idx
    }
}

impl Render<MessageRenderCtx> for Message {
    fn width(&self, ctx: &MessageRenderCtx) -> Option<usize> {
        let len = self.payload.graphemes(true).count();
        match ctx.is_loop() {
            // Put the text to the right of the loop arrow + some spacing
            true => Some(len + 3 + 3),
            // Reserve enough space for the arrow + some spacing
            false => Some(len + 4),
        }
    }

    fn height(&self, ctx: &MessageRenderCtx) -> Option<usize> {
        match ctx.is_loop() {
            true => Some(4),
            false => Some(3),
        }
    }

    fn render<D: Draw>(&self, canvas: &mut D, ctx: &MessageRenderCtx) -> DrawResult {
        let Rect { width, .. } = canvas.bounds();
        let payload = &self.payload;

        match ctx.is_loop() {
            true => {
                canvas.draw(
                    0,
                    0,
                    &["─┐".to_string(), " │".to_string(), "◀┘".to_string()],
                )?;
                canvas.draw(3, 1, &[payload.to_string()])?;
            }
            false => {
                let len = payload.graphemes(true).count();
                let left_padding = ((width - len) / 2) as usize;

                let mut arrow = "─".repeat(width - 2);
                match ctx.source_idx > ctx.target_idx {
                    true => {
                        arrow.push('─');
                        arrow.insert(0, '◀');
                    }
                    false => {
                        arrow.insert(0, '─');
                        arrow.push('▶');
                    }
                }

                canvas.draw(left_padding, 0, &[payload.to_string()])?;
                canvas.draw(0, 1, &[arrow])?;
            }
        }

        Ok(())
    }
}

pub struct Layout {
    solver: Solver,
    participants: Vec<Node<Arc<Participant>, BareRenderCtx>>,
    messages: Vec<Node<Message, MessageRenderCtx>>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            solver: Solver::new(),
            participants: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_participant(&mut self, participant: Arc<Participant>) {
        let spacing = 1.0;

        // Compute the left coordinate for the new participant
        let right = match self.participants.last() {
            Some(node) => node.vars.right() + spacing,
            None => Expression::from(0.0),
        };

        // Setup the initial constraints
        let render_ctx = BareRenderCtx {};
        let vars = Vars::new();
        let constraints = vec![
            vars.left | GE(REQUIRED) | right,
            vars.top | GE(REQUIRED) | 0.0,
            vars.width
                | EQ(REQUIRED)
                | participant
                    .width(&render_ctx)
                    .expect("Participant missing width") as f32,
            vars.height
                | EQ(REQUIRED)
                | participant
                    .height(&render_ctx)
                    .expect("Participant missing height") as f32,
        ];

        self.participants.push(Node {
            vars,
            constraints,
            data: participant,
            _phantom: PhantomData,
        });
    }

    pub fn add_message(&mut self, message: Message) {
        // Fetch the source and target nodes
        let mut participant_nodes = self.participants.iter_mut();

        // Find the participant to the left of the message
        let source_or_target =
            |p: &&mut Node<_, _>| p.data == message.source || p.data == message.target;
        let left_participant = participant_nodes.find(source_or_target).unwrap();

        // Find the participant to the right of the message. In case of loops, take just the
        // consecutive participant from the iterator.
        let right_participant = {
            let next_node = participant_nodes.next();
            let target_node = participant_nodes.find(source_or_target);
            next_node.filter(source_or_target).or(target_node)
        };

        // Compute the top coordinate for the new message
        let top = self
            .messages
            .last()
            .map(|msg| msg.vars.bottom())
            .or_else(|| Some(left_participant.vars.bottom()))
            .unwrap();

        // Construct the render context used to calculate the message with and height
        let render_ctx = MessageRenderCtx {
            source_idx: 0,
            target_idx: right_participant
                .as_ref()
                .map(|p| !(left_participant.data == p.data) as usize)
                .unwrap_or(0),
        };

        // Constraint the new message bounds in relation to the participants
        let vars = Vars::new();
        let mut constraints = vec![
            vars.top | EQ(REQUIRED) | top,
            vars.left | EQ(REQUIRED) | (left_participant.vars.center() + 1.0),
            vars.width
                | GE(REQUIRED)
                | message.width(&render_ctx).expect("Message missing width") as f64,
            vars.height
                | EQ(REQUIRED)
                | message.height(&render_ctx).expect("Message missing height") as f64,
        ];

        if let Some(right_participant) = right_participant {
            // Stretch the message to all the way to the participant on the right
            constraints.push(vars.right() | EQ(REQUIRED) | right_participant.vars.center());

            // Make space for the new message
            right_participant
                .constraints
                .push(right_participant.vars.center() | GE(REQUIRED) | vars.right());
        }

        self.messages.push(Node {
            vars,
            constraints,
            data: message,
            _phantom: PhantomData,
        });
    }

    /// Renders the layout and consumes itself.
    pub fn render(mut self) -> String {
        self.solver
            .add_constraints(
                self.participants
                    .iter()
                    .flat_map(|node| &node.constraints)
                    .collect::<Vec<&Constraint>>(),
            )
            .expect("Could not add the constraints");
        self.solver
            .add_constraints(
                self.messages
                    .iter()
                    .flat_map(|node| &node.constraints)
                    .collect::<Vec<&Constraint>>(),
            )
            .expect("Could not add the constraints");

        // Compute the required canvas size
        let max_right = self
            .participants
            .iter()
            .map(|p| p.coords(&self.solver).right())
            .chain(self.messages.iter().map(|p| p.coords(&self.solver).right()))
            .max()
            .map(|right| right + 1)
            .unwrap_or(0);
        let max_bottom = self
            .participants
            .iter()
            .map(|p| p.coords(&self.solver).bottom())
            .chain(
                self.messages
                    .iter()
                    .map(|p| p.coords(&self.solver).bottom()),
            )
            .max()
            .map(|bottom| bottom + 3)
            .unwrap_or(0);

        // Render the layout
        let bare_ctx = BareRenderCtx {};

        // Setup the canvas
        let mut canvas = TextCanvas::new(max_right, max_bottom);

        // Draw the participants and their lifelines
        let lifeline = vec!["│".to_string(); max_bottom];
        for node in &self.participants {
            let coords = node.coords(&self.solver);
            canvas
                .draw(coords.center(), 0, &lifeline)
                .expect("Draw failed");

            node.render(
                &mut canvas.region(coords.left, coords.top, coords.width, coords.height),
                &bare_ctx,
            )
            .expect("Draw failed");
            node.render(
                &mut canvas.region(coords.left, max_bottom - 3, coords.width, coords.height),
                &bare_ctx,
            )
            .expect("Draw failed");
        }

        // Draw the messages
        for node in &self.messages {
            let coords = node.coords(&self.solver);
            let source_idx = self
                .participants
                .iter()
                .position(|p| p.data == node.data.source)
                .unwrap();
            let target_idx = self
                .participants
                .iter()
                .position(|p| p.data == node.data.target)
                .unwrap();
            node.render(
                &mut canvas.region(coords.left, coords.top, coords.width, coords.height),
                &MessageRenderCtx {
                    source_idx,
                    target_idx,
                },
            )
            .expect("Draw failed");
        }
        canvas.content()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let mut layout = Layout::new();
        let participant_alice = Arc::from(Participant {
            id: "alice".into(),
            name: "Alice".into(),
        });
        let participant_bob = Arc::from(Participant {
            id: "bob".into(),
            name: "Bob".into(),
        });
        layout.add_participant(participant_alice.clone());
        layout.add_participant(participant_bob.clone());
        layout.add_message(Message {
            source: participant_alice.clone(),
            target: participant_bob.clone(),
            payload: "hello".to_string(),
        });
        layout.add_message(Message {
            source: participant_bob.clone(),
            target: participant_alice.clone(),
            payload: "hello back".to_string(),
        });
        layout.add_message(Message {
            source: participant_bob.clone(),
            target: participant_bob.clone(),
            payload: "who am i?".to_string(),
        });
        let output = layout.render();
        assert!(output.len() > 0);
    }
}
