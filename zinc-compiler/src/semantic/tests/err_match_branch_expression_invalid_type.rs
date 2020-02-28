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
    let scrutinee = 42;
    let result = match scrutinee {
        0 => false,
        1 => 0,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchExpressionInvalidType(
            Location::new(6, 14),
            Type::boolean().to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Location::new(5, 14),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
