use crate::layout;
use crate::parser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;

pub fn render(diag: parser::SequenceDiagram) -> String {
    // Gather all the unique participants in the following order:
    // 1. Nodes with explicit aliases (this enables easy reordering by moving around aliases)
    // 2. The rest of the participants in the order they appear
    let ordered_participants = {
        let mut participants = Vec::new();
        diag.aliases
            .iter()
            .for_each(|a| participants.push(&a.id[..]));
        diag.messages.iter().for_each(|m| {
            participants.push(&m.source[..]);
            participants.push(&m.target[..]);
        });

        // Deduplicate the participants
        let mut unique_participants: HashSet<&str> = HashSet::from_iter(participants.clone());
        let participants: Vec<&str> = participants
            .clone()
            .iter()
            .cloned()
            .filter(|p| unique_participants.remove(p))
            .collect();
        participants
    };

    // Gather all the aliases. For the nodes without an explicit alias, set it to the alias to the
    // node ID
    let aliases = {
        let mut aliases = HashMap::new();
        diag.aliases.iter().for_each(|a| {
            aliases.insert(&a.id[..], &a.label[..]);
        });
        diag.messages.iter().for_each(|m| {
            aliases.entry(&m.source).or_insert(&m.source);
            aliases.entry(&m.target).or_insert(&m.target);
        });
        aliases
    };

    // Construct the layout participants
    let participants = {
        let mut participants = HashMap::new();
        ordered_participants.iter().for_each(|p| {
            participants.insert(
                p.to_string(),
                Arc::from(layout::Participant {
                    id: p.to_string(),
                    name: aliases.get(p).unwrap().to_string(),
                }),
            );
        });
        participants
    };

    // Construct the layout
    let mut layout = layout::Layout::new();
    for participant in ordered_participants {
        layout.add_participant(participants.get(participant).unwrap().clone());
    }
    for message in diag.messages {
        layout.add_message(layout::Message {
            source: participants.get(&message.source).unwrap().clone(),
            target: participants.get(&message.target).unwrap().clone(),
            payload: message.payload,
            edge_style: match message.edge_style {
                parser::EdgeStyle::Continuous => layout::EdgeStyle::Continuous,
                parser::EdgeStyle::Dashed => layout::EdgeStyle::Dashed,
            },
        })
    }

    layout.render()
}

#[cfg(test)]
mod test {
    use crate::parser;

    #[test]
    fn smoke() {
        let mut diag = parser::SequenceDiagram::default();
        diag.aliases.push(parser::Alias {
            id: "test1".to_string(),
            label: "label".to_string(),
        });
        diag.aliases.push(parser::Alias {
            id: "test2".to_string(),
            label: "label".to_string(),
        });
        super::render(diag);
    }
}
