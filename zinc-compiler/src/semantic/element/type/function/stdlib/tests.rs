//!
//! The built-in function tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
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
        FunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayTruncatingToBiggerSize(
            2, 4,
        )),
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
        FunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayPaddingToLesserSize(
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
        FunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayNewLengthInvalid(
            "18446744073709551616: u72".to_owned(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
