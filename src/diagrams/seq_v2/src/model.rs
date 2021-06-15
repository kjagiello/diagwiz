#![allow(dead_code)]

use crate::parser::ast;
use crate::utils::{Span, Spanned};
use std::collections::HashMap;
use std::sync::Arc;

type Attrs<'a> = HashMap<&'a str, &'a str>;

#[derive(Debug)]
pub struct Participant<'a> {
    pub ident: Spanned<&'a str>,
    pub label: Option<Spanned<&'a str>>,
    pub span: Span,
}

#[derive(Debug)]
pub enum Element<'a> {
    Message {
        source: Arc<Participant<'a>>,
        target: Arc<Participant<'a>>,
        payload: &'a str,
        span: Span,
    },
    Separator {
        body: Spanned<&'a str>,
        span: Span,
    },
}

#[derive(Debug)]
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
                    ast::Stmt::Participant { ident, span, attrs } => Some(Arc::from(Participant {
                        span: *span,
                        ident: Spanned {
                            value: ident.str,
                            span: ident.span,
                        },
                        label: attrs
                            .as_ref()
                            .map(|attrs| attrs.get_str("label"))
                            .unwrap_or(None),
                    })),
                    _ => None,
                })
                .collect(),
        }
    }
}
