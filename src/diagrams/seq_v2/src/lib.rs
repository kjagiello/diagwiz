extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

use diagram_base::TransformError;

pub fn transform(_input: &str) -> Result<String, TransformError> {
    Ok("hej".into())
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
        let result = super::transform(data).unwrap();
        assert_eq!(result.contains("hej"), true);
    }
}
