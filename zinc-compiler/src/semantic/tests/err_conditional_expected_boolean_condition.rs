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
    if 42 { 1 } else { 2 }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ConditionalExpectedBooleanCondition {
            location: Location::new(3, 8),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        },
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
