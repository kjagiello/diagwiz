#![allow(dead_code)]

use crate::parser::ast;
use crate::utils::Span;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

type Attrs<'a> = HashMap<&'a str, &'a str>;

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
    pub ident: Spanned<&'a str>,
    pub label: Option<Spanned<&'a str>>,
    pub span: Span,
}

pub enum Element<'a> {
    Message {
        source: Arc<Participant<'a>>,
        target: Arc<Participant<'a>>,
        attrs: HashMap<Spanned<&'a str>, Spanned<&'a str>>,
        span: Span,
    },
    Separator {
        body: Spanned<&'a str>,
        span: Span,
    },
}

pub struct SeqDiag<'a> {
    pub participants: Vec<Arc<Participant<'a>>>,
    pub elements: Vec<Element<'a>>,
}

impl<'a> SeqDiag<'a> {
    pub fn from_tree(tree: ast::Tree<'a>) -> Self {
        Self {
            elements: vec![],
            participants: tree
                .stmts
                .iter()
                .filter_map(|stmt| match stmt {
                    ast::Stmt::Participant { ident, attrs, span } => Some(Arc::from(Participant {
                        span: span.clone(),
                        ident: Spanned {
                            value: ident.str,
                            span: ident.span,
                        },
                        label: None,
                    })),
                    _ => None,
                })
                .collect(),
        }
    }
}
