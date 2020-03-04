//!
//! The integer value element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::Error as SemanticError;

#[test]
fn error_element_value_integer_types_mismatch_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 == integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchEquals(
            Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_not_equals() {
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
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_greater_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 >= integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchGreaterEquals(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_lesser_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 <= integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchLesserEquals(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_greater() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 > integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchGreater(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_lesser() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 < integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchLesser(
            Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_addition() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 + integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchAddition(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_subtraction() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 - integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchSubtraction(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_multiplication() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_division() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 / integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchDivision(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_integer_types_mismatch_remainder() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 % integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchRemainder(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
