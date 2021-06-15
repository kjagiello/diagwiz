extern crate pest;
#[macro_use]
extern crate pest_derive;

mod model;
mod parser;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        let data = r#"a; b[label="alice"];"#;
        let tree = parser::parse(data).unwrap();
        let diag = model::SeqDiag::from_tree(tree);
        assert_eq!(diag.participants.len(), 2);
    }
}
