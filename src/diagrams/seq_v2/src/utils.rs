use std::ops::Deref;

// TODO: extract into a separate utility crate
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
