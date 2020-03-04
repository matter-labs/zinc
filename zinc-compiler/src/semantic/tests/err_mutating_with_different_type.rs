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
    let mut result = 42;
    result = false;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MutatingWithDifferentType {
        location: Location::new(4, 12),
        expected: Type::boolean().to_string(),
        found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
    }));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
