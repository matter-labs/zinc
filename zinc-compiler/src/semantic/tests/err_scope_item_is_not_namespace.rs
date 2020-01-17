//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::semantic::ScopeError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
const NOT_NAMESPACE: u8 = 42;

fn main(input: (), witness: ()) {
    let result = NOT_NAMESPACE::UNDEFINED;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(5, 18),
        ScopeError::ItemIsNotNamespace("NOT_NAMESPACE".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
