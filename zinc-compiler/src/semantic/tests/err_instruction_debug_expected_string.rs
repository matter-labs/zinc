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
fn main() {
    dbg!(42);
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::InstructionDebugExpectedString(Location::new(3, 9), "42: u8".to_string()),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
