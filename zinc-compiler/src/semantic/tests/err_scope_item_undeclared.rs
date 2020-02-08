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
fn main() {
    result = 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(3, 5),
        ScopeError::ItemUndeclared("result".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
