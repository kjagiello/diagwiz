use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Represents a source code span
#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

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
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum ArrowHead {
    Solid,
}

/// Represents an arrow line of a message
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum ArrowLine {
    Solid,
    Dashed,
}

/// Represents a message arrow
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Arrow {
    pub span: Span,
    pub head_left: Option<ArrowHead>,
    pub head_right: Option<ArrowHead>,
    pub line: ArrowLine,
}

// Attributes
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Atom<'a> {
    pub span: Span,
    pub value: AtomValue<'a>,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum AtomValue<'a> {
    Boolean(bool),
    String(&'a str),
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum AttrValue<'a> {
    Atom(Atom<'a>),
    /// Attributes can be implicitly set to true by only providing the attribute key
    ImplicitTrue,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct Attrs<'a> {
    pub span: Span,
    pub data: HashMap<Str<'a>, AttrValue<'a>>,
}

/// Statements that make up the AST
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
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
