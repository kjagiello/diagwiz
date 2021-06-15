extern crate pest;
#[macro_use]
extern crate pest_derive;

mod model;
mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke() {
        let data = r#"a; b[label="bob"];"#;
        let tree = parser::parse(data).unwrap();
        let diag = model::SeqDiag::from_tree(tree);
        assert_eq!(diag.participants.len(), 2);
        assert_eq!(*diag.participants[0].ident, "a");
        assert_eq!(diag.participants[0].label.as_deref(), None);
        assert_eq!(*diag.participants[1].ident, "b");
        assert_eq!(diag.participants[1].label.as_deref(), Some(&"bob"));
    }
}
