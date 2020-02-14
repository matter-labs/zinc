//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
enum Jabberwocky {
    Gone = 42,
}

fn main() {
    let really = Jabberwocky::Exists;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(7, 31),
        ScopeError::ItemUndeclared("Exists".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
