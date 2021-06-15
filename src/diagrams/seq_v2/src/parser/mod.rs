// TODO: remove
#![allow(dead_code)]

use std::collections::HashMap;

use pest::error::Error;
use pest::Parser;

use utils::Span;

#[cfg(test)]
mod tests;

pub(crate) mod ast;
pub(crate) mod utils;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct SeqParser;

type ParseResult<T> = Result<T, Error<Rule>>;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    SyntaxError(String),
}

impl From<Error<Rule>> for ParserError {
    fn from(err: Error<Rule>) -> ParserError {
        let renamed_err = err.renamed_rules(|rule| match *rule {
            Rule::WHITESPACE => "whitespace".to_owned(),
            Rule::COMMENT => "comment".to_owned(),
            Rule::eoi => "end of input".to_owned(),
            Rule::_keyword => "keyword".to_owned(),
            Rule::keyword => "keyword".to_owned(),
            Rule::identifier => "identifier".to_owned(),
            Rule::string => "string".to_owned(),
            Rule::string_inner => "string".to_owned(),
            Rule::string_char => "character".to_owned(),
            Rule::edge => "message arrow".to_owned(),
            Rule::boolean => "boolean".to_owned(),
            Rule::atom => "atom".to_owned(),
            Rule::attr => "attribute".to_owned(),
            Rule::attrs => "attributes".to_owned(),
            Rule::participant => "participant".to_owned(),
            Rule::message => "message".to_owned(),
            Rule::separator => "separator".to_owned(),
            Rule::stmt => "statement".to_owned(),
            Rule::main => "main".to_owned(),
        });
        ParserError::SyntaxError(renamed_err.to_string())
    }
}

impl<'i> From<pest::Span<'i>> for Span {
    fn from(span: pest::Span) -> Span {
        Span {
            start: span.start(),
            end: span.end(),
        }
    }
}

pub fn parse(input: &str) -> Result<ast::Tree, ParserError> {
    let mut stmts: Vec<ast::Stmt> = vec![];
    let pairs = SeqParser::parse(Rule::main, input)?;
    for pair in pairs {
        stmts.push(parse_stmt(pair)?);
    }
    Ok(ast::Tree { stmts })
}

fn parse_stmt(pair: pest::iterators::Pair<Rule>) -> ParseResult<ast::Stmt> {
    match pair.as_rule() {
        Rule::participant => {
            let span = pair.as_span();
            let mut inner_rules = pair.into_inner();

            let ident = inner_rules.next().unwrap();
            let attrs = match inner_rules.next() {
                Some(rule) => Some(parse_attrs(rule)?),
                None => None,
            };

            Ok(ast::Stmt::Participant {
                span: span.into(),
                ident: parse_ident(ident)?,
                attrs,
            })
        }
        Rule::message => {
            let span = pair.as_span();
            let mut inner_rules = pair.into_inner();

            let ident1 = inner_rules.next().unwrap();
            let arrow = inner_rules.next().unwrap();
            let ident2 = inner_rules.next().unwrap();
            let attrs = match inner_rules.next() {
                Some(rule) => Some(parse_attrs(rule)?),
                None => None,
            };
            let (head_left, head_right) = parse_arrow_heads(arrow.as_str())?;
            Ok(ast::Stmt::Message {
                span: span.into(),
                ident1: parse_ident(ident1)?,
                ident2: parse_ident(ident2)?,
                arrow: ast::Arrow {
                    span: arrow.as_span().into(),
                    line: parse_arrow_line(arrow.as_str())?,
                    head_left,
                    head_right,
                },
                attrs,
            })
        }
        rule => unreachable!("Unhandled rule: {:#?}", rule),
    }
}

fn parse_ident(pair: pest::iterators::Pair<Rule>) -> ParseResult<ast::Str> {
    Ok(ast::Str {
        span: pair.as_span().into(),
        str: pair.as_str(),
    })
}

fn parse_string(pair: pest::iterators::Pair<Rule>) -> ParseResult<ast::Str> {
    Ok(ast::Str {
        span: pair.as_span().into(),
        str: pair.as_str(),
    })
}

fn parse_arrow_heads(line: &str) -> ParseResult<(Option<ast::ArrowHead>, Option<ast::ArrowHead>)> {
    let result = match line {
        "-->" => (None, Some(ast::ArrowHead::Solid)),
        "->" => (None, Some(ast::ArrowHead::Solid)),
        "<--" => (Some(ast::ArrowHead::Solid), None),
        "<-" => (Some(ast::ArrowHead::Solid), None),
        v => unreachable!("Unsupported arrow head: {:#?}", v),
    };
    Ok(result)
}

fn parse_arrow_line(line: &str) -> ParseResult<ast::ArrowLine> {
    let result = match line {
        "-->" => ast::ArrowLine::Dashed,
        "->" => ast::ArrowLine::Solid,
        "<--" => ast::ArrowLine::Dashed,
        "<-" => ast::ArrowLine::Solid,
        v => unreachable!("Unsupported arrow line: {:#?}", v),
    };
    Ok(result)
}

fn parse_attrs(pair: pest::iterators::Pair<Rule>) -> ParseResult<ast::Attrs> {
    match pair.as_rule() {
        Rule::attrs => {
            let span = pair.as_span();
            let result = ast::Attrs {
                span: span.into(),
                data: pair
                    .into_inner()
                    .map(|apair| {
                        let mut inner_rules = apair.into_inner();
                        let key = parse_string(inner_rules.next().unwrap())?;
                        let value = parse_attr_value(inner_rules.next())?;
                        Ok((key, value))
                    })
                    .collect::<ParseResult<HashMap<_, _>>>()?,
            };
            Ok(result)
        }
        rule => unreachable!("Unhandled attrs rule: {:#?}", rule),
    }
}

fn parse_attr_value(pair: Option<pest::iterators::Pair<Rule>>) -> ParseResult<ast::AttrValue> {
    let result = match pair {
        Some(v) => ast::AttrValue::Atom(parse_atom(v)?),
        None => ast::AttrValue::ImplicitTrue,
    };
    Ok(result)
}

fn parse_atom(pair: pest::iterators::Pair<Rule>) -> ParseResult<ast::Atom> {
    let span = pair.as_span().into();
    let inner_pair = pair
        .into_inner()
        .next()
        .unwrap_or_else(|| panic!("Missing atom inner pair"));
    let value = match inner_pair.as_rule() {
        Rule::string_inner => ast::AtomValue::String(inner_pair.as_str()),
        Rule::boolean => ast::AtomValue::Boolean(parse_boolean(inner_pair)?),
        rule => unreachable!("Unhandled atom rule: {:#?}", rule),
    };
    Ok(ast::Atom { span, value })
}

fn parse_boolean(pair: pest::iterators::Pair<Rule>) -> ParseResult<bool> {
    match pair.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        value => unreachable!("Invalid boolean value: {:#?}", value),
    }
}
