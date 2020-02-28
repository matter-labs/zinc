//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 * integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchMultiplication(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
