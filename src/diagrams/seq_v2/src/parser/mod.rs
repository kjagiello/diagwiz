// TODO: remove
#![allow(dead_code)]

mod ast;
use std::collections::HashMap;

use pest::error::Error;
use pest::Parser;

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

impl<'i> From<pest::Span<'i>> for ast::Span {
    fn from(span: pest::Span) -> ast::Span {
        ast::Span {
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
                ident: ast::Str {
                    span: ident.as_span().into(),
                    str: ident.as_str(),
                },
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

            Ok(ast::Stmt::Message {
                span: span.into(),
                ident1: ast::Str {
                    span: ident1.as_span().into(),
                    str: ident1.as_str(),
                },
                ident2: ast::Str {
                    span: ident2.as_span().into(),
                    str: ident2.as_str(),
                },
                arrow: ast::Arrow {
                    span: arrow.as_span().into(),
                    head_left: None,
                    head_right: None,
                },
                attrs,
            })
        }
        rule => unreachable!("Unhandled rule: {:#?}", rule),
    }
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

                        let key = {
                            let pair = inner_rules.next().unwrap();
                            ast::Str {
                                span: pair.as_span().into(),
                                str: pair.as_str(),
                            }
                        };
                        let value_pair = inner_rules.next();
                        let value = parse_attr_value(value_pair)?;
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
    println!("atom found: {:#?}", pair);
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_participant() {
        let data = r#"
        a;
        b [a=false];
        c [label="C", padding="1"];

        a->b [hide_arrow];
        b->c [label="test"];
        "#;
        let result = super::parse(data);
        match result {
            Ok(output) => println!("{:#?}", output),
            Err(super::ParserError::SyntaxError(e)) => println!("{}", e),
        };
        assert_eq!(true, false);
    }
}
