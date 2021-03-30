extern crate pest;
#[macro_use]
extern crate pest_derive;

use diagram_base::TransformError;

mod layout;
mod parser;
mod renderer;

impl From<parser::ParserError> for TransformError {
    fn from(err: parser::ParserError) -> TransformError {
        match err {
            parser::ParserError::SyntaxError(details) => TransformError::ParseError(details),
        }
    }
}

pub fn transform(input: String) -> Result<String, TransformError> {
    let diagram = parser::diagram(input)?;
    let output = renderer::render(diagram);
    Ok(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_smoke() {
        let data = r#"
        alias a="Foo"
        alias b="Bar"
        a->b:"hey"
        "#;
        let result = super::transform(String::from(data)).unwrap();
        assert_eq!(result.contains("Foo"), true);
        assert_eq!(result.contains("Bar"), true);
    }
}
