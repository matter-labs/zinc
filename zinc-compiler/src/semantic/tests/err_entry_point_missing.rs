//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn another() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointMissing));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
