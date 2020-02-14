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
    if true { 42 } else { false }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ConditionalBranchTypesMismatch(
            Location::new(3, 5),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Type::boolean().to_string(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
