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
    let result = 42;
    {
        let result = 69;
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(5, 9),
        ScopeError::ItemRedeclared("result".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
