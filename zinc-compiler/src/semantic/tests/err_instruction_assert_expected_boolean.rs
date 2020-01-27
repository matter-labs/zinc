//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    assert!(42);
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::InstructionAssertExpectedBoolean(Location::new(3, 12), "42: u8".to_string()),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
