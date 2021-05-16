use crate::parser;
use crate::parser::ast;
use maplit::hashmap;

#[test]
fn test_participants() {
    let data = "a; b;";
    let result = parser::parse(data).unwrap();
    assert_eq!(result.stmts.len(), 2);
    assert_eq!(
        result.stmts[0],
        ast::Stmt::Participant {
            span: ast::Span { start: 0, end: 1 },
            ident: ast::Str {
                span: ast::Span { start: 0, end: 1 },
                str: "a",
            },
            attrs: None,
        }
    );
    assert_eq!(
        result.stmts[1],
        ast::Stmt::Participant {
            span: ast::Span { start: 3, end: 4 },
            ident: ast::Str {
                span: ast::Span { start: 3, end: 4 },
                str: "b"
            },
            attrs: None
        }
    );
}

#[test]
fn test_attr_implicit_true() {
    let data = "a[compact];";
    let result = parser::parse(data).unwrap();
    assert_eq!(result.stmts.len(), 1);
    assert_eq!(
        result.stmts[0],
        ast::Stmt::Participant {
            span: ast::Span { start: 0, end: 10 },
            ident: ast::Str {
                span: ast::Span { start: 0, end: 1 },
                str: "a"
            },
            attrs: Some(ast::Attrs {
                span: ast::Span { start: 1, end: 10 },
                data: hashmap! {
                    ast::Str {
                        span: ast::Span { start: 2, end: 9 },
                        str: "compact",
                    } => ast::AttrValue::ImplicitTrue
                }
            })
        },
    );
}

#[test]
fn test_attr_explicit_boolean() {
    let data = "a[compact=false, bold=true];";
    let result = parser::parse(data).unwrap();
    assert_eq!(result.stmts.len(), 1);
    assert_eq!(
        result.stmts[0],
        ast::Stmt::Participant {
            span: ast::Span { start: 0, end: 27 },
            ident: ast::Str {
                span: ast::Span { start: 0, end: 1 },
                str: "a"
            },
            attrs: Some(ast::Attrs {
                span: ast::Span { start: 1, end: 27 },
                data: hashmap! {
                    ast::Str {
                        span: ast::Span {
                            start: 17,
                            end: 21
                        },
                        str: "bold"
                    } => ast::AttrValue::Atom(
                        ast::Atom {
                            span: ast::Span {
                                start: 22,
                                end: 26,
                            },
                            value: ast::AtomValue::Boolean(true)
                        }
                    ),
                    ast::Str {
                        span: ast::Span {
                            start: 2,
                            end: 9
                        },
                        str: "compact"
                    } => ast::AttrValue::Atom(
                        ast::Atom {
                            span: ast::Span {
                                start: 10,
                                end: 15
                            },
                            value: ast::AtomValue::Boolean(false)
                        }
                    )
                }
            }),
        },
    );
}

#[test]
fn test_attr_str() {
    let data = "a[compact=\"yes\"];";
    let result = parser::parse(data).unwrap();
    assert_eq!(result.stmts.len(), 1);
    assert_eq!(
        result.stmts[0],
        ast::Stmt::Participant {
            span: ast::Span { start: 0, end: 16 },
            ident: ast::Str {
                span: ast::Span { start: 0, end: 1 },
                str: "a"
            },
            attrs: Some(ast::Attrs {
                span: ast::Span { start: 1, end: 16 },
                data: hashmap! {
                    ast::Str {
                        span: ast::Span { start: 2, end: 9 },
                        str: "compact",
                    } => ast::AttrValue::Atom(
                        ast::Atom {
                            span: ast::Span {
                                start: 10,
                                end: 15
                            },
                            value: ast::AtomValue::String("yes")
                        }
                    )
                }
            })
        },
    );
}

#[test]
fn test_message() {
    let data = "a->b [];";
    let result = parser::parse(data).unwrap();
    assert_eq!(result.stmts.len(), 1);
    assert_eq!(
        result.stmts[0],
        ast::Stmt::Message {
            span: ast::Span { start: 0, end: 7 },
            ident1: ast::Str {
                span: ast::Span { start: 0, end: 1 },
                str: "a"
            },
            ident2: ast::Str {
                span: ast::Span { start: 3, end: 4 },
                str: "b"
            },
            arrow: ast::Arrow {
                span: ast::Span { start: 1, end: 3 },
                head_left: None,
                head_right: Some(ast::ArrowHead::Solid),
                line: ast::ArrowLine::Solid,
            },
            attrs: Some(ast::Attrs {
                span: ast::Span { start: 5, end: 7 },
                data: hashmap! {}
            })
        }
    );
}
