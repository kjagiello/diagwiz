#![allow(dead_code)]

use crate::parser::ast;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

type Attrs<'a> = HashMap<&'a str, &'a str>;

// TODO: extract into a separate utility crate
#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<ast::Span> for Span {
    fn from(span: ast::Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

pub struct Participant<'a> {
    pub span: Span,
    pub ident: Spanned<&'a str>,
    pub label: Spanned<&'a str>,
}

pub enum Element<'a> {
    Message {
        span: Span,
        source: Arc<Participant<'a>>,
        target: Arc<Participant<'a>>,
        attrs: HashMap<Spanned<&'a str>, Spanned<&'a str>>,
    },
    Separator {
        span: Span,
        body: Spanned<&'a str>,
    },
}

pub struct SeqDiag<'a> {
    pub participants: Vec<Arc<Participant<'a>>>,
    pub elements: Vec<Element<'a>>,
}

impl<'a> SeqDiag<'a> {
    pub fn from_tree(tree: ast::Tree) -> Self {
        Self {
            elements: vec![],
            participants: tree
                .stmts
                .iter()
                .filter_map(|stmt| match stmt {
                    ast::Stmt::Participant { .. } => Some(stmt),
                    _ => None,
                })
                .collect(),
        }
    }
}
