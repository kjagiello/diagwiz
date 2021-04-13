use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub struct SequenceDiagramParser;

#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    /// The ID of the node
    pub id: String,
    /// The label of the node
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// The sender of the message
    pub source: String,
    /// The recipient of the message
    pub target: String,
    /// The edge label
    pub payload: String,
}

#[derive(Debug, Clone, Default)]
pub struct SequenceDiagram {
    /// List of aliases
    pub aliases: Vec<Alias>,
    /// List of messages
    pub messages: Vec<Message>,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    SyntaxError(String),
}

pub fn diagram(input: String) -> Result<SequenceDiagram, ParserError> {
    let ast = SequenceDiagramParser::parse(Rule::main, input.as_str());
    match ast {
        // TODO: rename
        Ok(mut ast) => {
            let mut diag = SequenceDiagram::default();
            for stmt in ast.next().unwrap().into_inner() {
                match stmt.as_rule() {
                    Rule::alias => {
                        // { ^"alias" ~ name ~ "=" ~ string }
                        let mut inner_rules = stmt.into_inner();

                        let name: &str = inner_rules.next().unwrap().as_str();
                        let value: &str = inner_rules.next().unwrap().as_str();

                        diag.aliases.push(Alias {
                            id: String::from(name),
                            label: String::from(value),
                        });
                    }
                    Rule::pair => {
                        // { name ~ "->" ~ name ~ ":" ~ string }
                        let mut inner_rules = stmt.into_inner();

                        let source: &str = inner_rules.next().unwrap().as_str();
                        let target: &str = inner_rules.next().unwrap().as_str();
                        let label: &str = match inner_rules.peek() {
                            Some(_) => inner_rules.next().unwrap().as_str(),
                            None => "",
                        };

                        diag.messages.push(Message {
                            source: String::from(source),
                            target: String::from(target),
                            payload: String::from(label),
                        });
                    }
                    Rule::EOI => (),
                    _ => unreachable!(),
                }
            }
            Ok(diag)
        }
        Err(e) => Err(ParserError::SyntaxError(e.to_string())),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_diagram_empty() {
        let data = "";
        let result = diagram(String::from(data)).unwrap();
        assert_eq!(result.aliases.len(), 0);
        assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn parse_comment() {
        let data = r#"
        # test
        // comment
        "#;
        let result = diagram(String::from(data)).unwrap();
        assert_eq!(result.aliases.len(), 0);
        assert_eq!(result.messages.len(), 0);
    }

    #[test]
    fn parse_empty_message() {
        let data = "a->b";
        let result = diagram(String::from(data)).unwrap();
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.messages[0].payload, "");
    }

    #[test]
    fn parse_message_payload_with_unicode() {
        let data = r#"a->b: "𩸽""#;
        let result = diagram(String::from(data)).unwrap();
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.messages[0].payload, "𩸽");
    }

    #[test]
    // TODO: ensure that we handle the escape sequences correctly
    #[should_panic]
    fn parse_message_payload_with_escape_sequences() {
        let data = "a->b: \"\\\"hello\\\"\"\n";
        let result = diagram(String::from(data)).unwrap();
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.messages[0].payload, "\"hello\"");
    }

    #[test]
    fn disallows_keyword_identifiers() {
        let data = "alias alias = \"aliasson\"";
        let result = diagram(String::from(data));
        assert!(result.is_err());
    }

    #[test]
    fn disallows_idedntifiers_with_numeric_prefix() {
        let data = "alias 1a = \"b\"";
        let result = diagram(String::from(data));
        assert!(result.is_err());
    }

    #[test]
    fn allows_identifier_with_keyword_substring() {
        let data = "alias aliassson = \"aliasson\"";
        let result = diagram(String::from(data));
        assert!(!result.is_err());
    }

    #[test]
    fn allows_underscores_in_identifiers() {
        let data = "alias _a_b_ = \"c\"";
        let result = diagram(String::from(data));
        assert!(!result.is_err());
    }

    #[test]
    fn requires_a_space_after_alias_keyword() {
        let data = "aliasabc = \"d\"";
        let result = diagram(String::from(data));
        assert!(result.is_err());
    }
}
