//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerValueError;
use crate::semantic::Type;
use crate::semantic::ValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 != integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchNotEquals(
                Type::new_integer_unsigned(64).to_string(),
                Type::new_integer_unsigned(128).to_string(),
            ),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
