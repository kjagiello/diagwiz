use std::ops::Deref;

// TODO: extract into a separate utility crate
#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}
