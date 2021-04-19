use std::iter::FromIterator;

type CanvasBuffer = Vec<Vec<char>>;

#[derive(Debug)]
pub struct Rect {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    fn right(&self) -> usize {
        self.left + self.width
    }

    fn bottom(&self) -> usize {
        self.top + self.height
    }

    fn contains(&self, rect: &Rect) -> bool {
        let horizontal = self.left..(self.right() + 1);
        let vertical = self.top..(self.bottom() + 1);
        horizontal.contains(&rect.left)
            && horizontal.contains(&rect.right())
            && vertical.contains(&rect.top)
            && vertical.contains(&rect.bottom())
    }
}

pub struct TextCanvas {
    bounds: Rect,
    buffer: CanvasBuffer,
}

pub struct TextCanvasRegion<'a> {
    bounds: Rect,
    buffer: &'a mut CanvasBuffer,
}

#[derive(Debug, PartialEq)]
pub enum DrawError {
    HorizontalOverflow,
    VerticalOverflow,
}

pub type DrawResult = Result<(), DrawError>;

pub trait Draw {
    fn bounds(&self) -> &Rect;
    fn buffer_mut(&mut self) -> &mut CanvasBuffer;

    fn draw(&mut self, left: usize, top: usize, rows: &[&str]) -> DrawResult {
        // TODO: ensure that the draw cannot happen outside of the region
        let bounds = self.bounds();
        let max_length = rows.iter().map(|s| s.chars().count()).max().unwrap();
        if max_length > bounds.width - left {
            Err(DrawError::HorizontalOverflow)
        } else if rows.len() > bounds.height - top {
            Err(DrawError::VerticalOverflow)
        } else {
            let left = left + bounds.left;
            let top = top + bounds.top;
            let buffer = self.buffer_mut();
            let pairs = buffer[top..top + rows.len()].iter_mut().zip(rows);
            for (brow, drow) in pairs {
                brow.splice(left..left + drow.chars().count(), drow.chars());
            }
            Ok(())
        }
    }
}

impl Draw for TextCanvas {
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn buffer_mut(&mut self) -> &mut CanvasBuffer {
        &mut self.buffer
    }
}

impl<'a> Draw for TextCanvasRegion<'a> {
    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn buffer_mut(&mut self) -> &mut CanvasBuffer {
        &mut self.buffer
    }
}

impl TextCanvas {
    pub fn new(width: usize, height: usize) -> TextCanvas {
        TextCanvas {
            bounds: Rect {
                left: 0,
                top: 0,
                width,
                height,
            },
            buffer: (0..height).map(|_| vec![' '; width]).collect(),
        }
    }

    pub fn region(
        &mut self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> TextCanvasRegion {
        let bounds = Rect {
            left,
            top,
            width,
            height,
        };
        if !self.bounds.contains(&bounds) {
            panic!(
                "Creating invalid region. Canvas bounds: {:?}, region bounds: {:?}",
                self.bounds, bounds
            );
        }
        TextCanvasRegion {
            bounds,
            buffer: &mut self.buffer,
        }
    }

    pub fn content(&self) -> String {
        self.buffer
            .iter()
            .map(String::from_iter)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_text() {
        let mut canvas = TextCanvas::new(5, 5);
        let data = ["123", "456", "789"];
        canvas.draw(0, 0, &data).expect("Draw failed");
        canvas.draw(2, 2, &data).expect("Draw failed");
        #[rustfmt::skip]
        let expected = vec![
            "123  ",
            "456  ",
            "78123",
            "  456",
            "  789",
        ];
        assert_eq!(canvas.content(), expected.join("\n"));
    }

    #[test]
    fn horizontal_overflow() {
        let data = ["123", "456", "789"];
        let mut canvas = TextCanvas::new(2, 3);
        assert_eq!(canvas.draw(0, 0, &data), Err(DrawError::HorizontalOverflow));
    }

    #[test]
    fn vertical_overflow() {
        let data = ["123", "456", "789"];
        let mut canvas = TextCanvas::new(3, 2);
        assert_eq!(canvas.draw(0, 0, &data), Err(DrawError::VerticalOverflow));
    }

    #[test]
    fn unicode_boundaries() {
        // Draw a 4 byte character on the canvas, then try to replace it with a 1 byte character
        // and ensure that the replacement process respects the character boundaries.
        let mut canvas = TextCanvas::new(1, 1);

        let data = ["𩸽"];
        canvas.draw(0, 0, &data).expect("Draw failed");
        assert_eq!(canvas.content(), data.join("\n"));

        let data = ["a"];
        canvas.draw(0, 0, &data).expect("Draw failed");
        assert_eq!(canvas.content(), data.join("\n"));
    }

    #[test]
    fn region_draw() {
        let mut canvas = TextCanvas::new(3, 1);

        // Paint in a one pixel region
        let mut region1 = canvas.region(0, 0, 1, 1);
        let data = ["a"];
        region1.draw(0, 0, &data).expect("Draw failed");

        // Paint in a different one pixel region twice
        let mut region2 = canvas.region(2, 0, 1, 1);
        let data = ["z"];
        region2.draw(0, 0, &data).expect("Draw failed");
        let data = ["b"];
        region2.draw(0, 0, &data).expect("Draw failed");

        // Inspect the canvas content
        let expected = ["a b"];
        assert_eq!(canvas.content(), expected.join("\n"));
    }

    #[test]
    fn region_horizontal_overflow() {
        let mut canvas = TextCanvas::new(3, 3);
        let mut region = canvas.region(0, 0, 1, 1);
        let data = ["a"];
        assert_eq!(region.draw(1, 0, &data), Err(DrawError::HorizontalOverflow));
    }

    #[test]
    fn region_vertical_overflow() {
        let mut canvas = TextCanvas::new(3, 3);
        let mut region = canvas.region(0, 0, 1, 1);
        let data = ["a"];
        assert_eq!(region.draw(0, 1, &data), Err(DrawError::VerticalOverflow));
    }
}
