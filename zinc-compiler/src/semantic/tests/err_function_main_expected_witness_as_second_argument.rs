//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: u8, not_witness: u8) -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionMainExpectedWitnessAsSecondArgument("not_witness".to_owned()),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
