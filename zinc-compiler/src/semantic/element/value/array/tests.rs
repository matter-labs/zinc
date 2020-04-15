//!
//! The array value element tests.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_pushing_invalid_type() {
    let input = r#"
fn main() {
    let array = [1, false];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Value(ValueError::Array(ArrayValueError::PushingInvalidType {
            expected: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            found: Type::boolean().to_string(),
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_start_out_of_range() {
    let input = r#"
fn main() {
    [1, 2, 3, 4, 5][-(1 as i8) .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Value(ValueError::Array(ArrayValueError::SliceStartOutOfRange {
            start: BigInt::from(-1).to_string(),
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_end_out_of_range() {
    let input = r#"
fn main() {
    [1, 2, 3, 4, 5][0 .. 6];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Value(ValueError::Array(ArrayValueError::SliceEndOutOfRange {
            end: BigInt::from(6).to_string(),
            size: 5,
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_end_lesser_than_start() {
    let input = r#"
fn main() {
    [1, 2, 3, 4, 5][2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Value(ValueError::Array(
            ArrayValueError::SliceEndLesserThanStart {
                start: BigInt::from(2).to_string(),
                end: BigInt::from(1).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
