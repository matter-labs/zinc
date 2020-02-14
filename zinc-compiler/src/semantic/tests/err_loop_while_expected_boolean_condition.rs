//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut sum = 0;
    for i in 0..10 while 42 {
        sum = sum + i;
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::LoopWhileExpectedBooleanCondition(
            Location::new(4, 26),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
