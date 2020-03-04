//!
//! The built-in function tests.
//!

#![cfg(test)]

use std::str::FromStr;

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionError;
use crate::semantic::Error as SemanticError;

#[test]
fn error_array_truncating_to_bigger_size() {
    let input = r#"
fn main() -> [u8; 4] {
    std::array::truncate([1, 2], 4)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(3, 25),
        FunctionError::StandardLibrary(
            StandardLibraryFunctionError::array_truncating_to_bigger_size(2, 4),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_padding_to_bigger_size() {
    let input = r#"
fn main() -> [u8; 4] {
    std::array::pad([1, 2, 3, 4], 2, 0)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(3, 20),
        FunctionError::StandardLibrary(StandardLibraryFunctionError::array_padding_to_lesser_size(
            4, 2,
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_new_length_invalid() {
    let input = r#"
fn main() -> [u8; 4] {
    std::array::truncate([1], 0x1_00000000_00000000)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(3, 25),
        FunctionError::StandardLibrary(StandardLibraryFunctionError::array_new_length_invalid(
            IntegerConstant::new(
                BigInt::from_str("18446744073709551616")
                    .expect(crate::semantic::tests::PANIC_TEST_DATA),
                false,
                72,
            )
            .to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
