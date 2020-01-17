//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: u8, witness: u8, wrong: field) -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionMainExpectedTwoArguments(3),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
