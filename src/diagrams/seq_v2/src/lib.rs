extern crate pest;
#[macro_use]
extern crate pest_derive;

mod model;
mod parser;
mod utils;

use diagram_base::TransformError;

pub fn transform(_input: &str) -> Result<String, TransformError> {
    Ok("hej".into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_smoke() {
        let data = r#"a; b[label="alice"];"#;
        let tree = crate::parser::parse(data).unwrap();
        let diag = crate::model::SeqDiag::from_tree(tree);
        assert_eq!(diag.participants.len(), 2);
    }
}
