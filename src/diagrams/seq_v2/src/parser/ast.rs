use crate::utils::{Span, Spanned};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Contains a diagram AST
#[derive(Debug, Default)]
pub struct Tree<'a> {
    pub stmts: Vec<Stmt<'a>>,
}

/// Represents a string
#[derive(Debug)]
pub struct Str<'a> {
    pub span: Span,
    pub str: &'a str,
}

impl<'a> Str<'a> {
    fn spanless(value: &'a str) -> Str<'a> {
        Str {
            span: Span { start: 0, end: 0 },
            str: value,
        }
    }
}

impl<'a> Hash for Str<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.str.hash(state);
    }
}

impl<'a> PartialEq for Str<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl<'a> Eq for Str<'a> {}

/// Represent an arrowhead of a line of a message
#[derive(Debug, PartialEq)]
pub enum ArrowHead {
    Solid,
}

/// Represents an arrow line of a message
#[derive(Debug, PartialEq)]
pub enum ArrowLine {
    Solid,
    Dashed,
}

/// Represents a message arrow
#[derive(Debug, PartialEq)]
pub struct Arrow {
    pub span: Span,
    pub head_left: Option<ArrowHead>,
    pub head_right: Option<ArrowHead>,
    pub line: ArrowLine,
}

// Attributes
#[derive(Debug, PartialEq)]
pub struct Atom<'a> {
    pub span: Span,
    pub value: AtomValue<'a>,
}

#[derive(Debug, PartialEq)]
pub enum AtomValue<'a> {
    Boolean(bool),
    String(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum AttrValue<'a> {
    Atom(Atom<'a>),
    /// Attributes can be implicitly set to true by only providing the attribute key
    ImplicitTrue,
}

#[derive(Debug, PartialEq)]
pub struct Attrs<'a> {
    pub span: Span,
    pub data: HashMap<Str<'a>, AttrValue<'a>>,
}

impl<'a> Attrs<'a> {
    pub fn get_str(&self, key: &str) -> Option<Spanned<&'a str>> {
        // TODO: ensure right type in the attr value
        self.data
            .get(&Str::spanless(key))
            .map(|value| match value {
                AttrValue::Atom(atom) => Some(atom),
                _ => None,
            })
            .unwrap()
            .map(|atom| match atom.value {
                AtomValue::String(str) => Some(Spanned {
                    span: atom.span,
                    value: str,
                }),
                _ => None,
            })
            .unwrap_or(None)
    }
}

/// Statements that make up the AST
#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    /// Diagram participant with its display name
    Participant {
        span: Span,
        ident: Str<'a>,
        attrs: Option<Attrs<'a>>,
    },
    /// Message sent from one participant to another
    Message {
        span: Span,
        ident1: Str<'a>,
        ident2: Str<'a>,
        arrow: Arrow,
        attrs: Option<Attrs<'a>>,
    },
    /// Separates messages
    Separator { span: Span, text: &'a str },
}
