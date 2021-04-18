use pretty_assertions::assert_eq;
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct PrettyString<'a>(pub &'a str);

/// Make diff to display string as multi-line string
impl<'a> fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

macro_rules! assert_renders {
    ($input:expr, $output:expr$(,)*) => {{
        assert_eq!(
            PrettyString($input.unwrap().as_str()),
            PrettyString($output),
        );
    }};
}

#[test]
fn test_loop() {
    assert_renders!(
        diagram_seq::transform("a->a".to_string()),
        concat!(
            "┌────┐     \n",
            "│ a  │     \n",
            "└────┘     \n",
            "   │─┐     \n",
            "   │ │     \n",
            "   │◀┘     \n",
            "   │       \n",
            "┌────┐     \n",
            "│ a  │     \n",
            "└────┘     ",
        ),
    );
}

#[test]
fn test_two_participants_with_continuous_line() {
    assert_renders!(
        diagram_seq::transform("a->b".to_string()),
        concat!(
            "┌────┐ ┌────┐ \n",
            "│ a  │ │ b  │ \n",
            "└────┘ └────┘ \n",
            "   │      │   \n",
            "   │─────▶│   \n",
            "   │      │   \n",
            "┌────┐ ┌────┐ \n",
            "│ a  │ │ b  │ \n",
            "└────┘ └────┘ ",
        ),
    );
}

#[test]
fn test_two_participants_with_dashed_line() {
    assert_renders!(
        diagram_seq::transform("a-->b".to_string()),
        concat!(
            "┌────┐ ┌────┐ \n",
            "│ a  │ │ b  │ \n",
            "└────┘ └────┘ \n",
            "   │      │   \n",
            "   │-----▶│   \n",
            "   │      │   \n",
            "┌────┐ ┌────┐ \n",
            "│ a  │ │ b  │ \n",
            "└────┘ └────┘ ",
        ),
    );
}

#[test]
fn test_two_participants_with_message() {
    assert_renders!(
        diagram_seq::transform(
            r#"
            a->b: "hello world"
            "#
            .into()
        ),
        concat!(
            "┌────┐           ┌────┐ \n",
            "│ a  │           │ b  │ \n",
            "└────┘           └────┘ \n",
            "   │  hello world   │   \n",
            "   │───────────────▶│   \n",
            "   │                │   \n",
            "┌────┐           ┌────┐ \n",
            "│ a  │           │ b  │ \n",
            "└────┘           └────┘ ",
        ),
    );
}

#[test]
fn test_complex() {
    assert_renders!(
        diagram_seq::transform(
            r#"
            alias a = "Alice"
            alias b = "Bob"
            alias c = "Charlie"

            a->c: "hello world"
            b->a: "hello there"
            c-->a: "hello back"
            c->b: "hello back too"
            b->b: "hello?"
            "#
            .into()
        ),
        concat!(
            "┌────────┐        ┌──────┐         ┌──────────┐ \n",
            "│ Alice  │        │ Bob  │         │ Charlie  │ \n",
            "└────────┘        └──────┘         └──────────┘ \n",
            "     │            hello world            │      \n",
            "     │──────────────────────────────────▶│      \n",
            "     │                │                  │      \n",
            "     │  hello there   │                  │      \n",
            "     │◀───────────────│                  │      \n",
            "     │                │                  │      \n",
            "     │            hello back             │      \n",
            "     │◀----------------------------------│      \n",
            "     │                │                  │      \n",
            "     │                │  hello back too  │      \n",
            "     │                │◀─────────────────│      \n",
            "     │                │                  │      \n",
            "     │                │─┐                │      \n",
            "     │                │ │ hello?         │      \n",
            "     │                │◀┘                │      \n",
            "     │                │                  │      \n",
            "┌────────┐        ┌──────┐         ┌──────────┐ \n",
            "│ Alice  │        │ Bob  │         │ Charlie  │ \n",
            "└────────┘        └──────┘         └──────────┘ ",
        ),
    );
}
