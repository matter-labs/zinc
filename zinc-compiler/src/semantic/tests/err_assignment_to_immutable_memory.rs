//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let result = 42;
    result = 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::AssignmentToImmutableMemory(
        Location::new(4, 5),
        "0".to_string(),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
