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
    assert();
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionInstructionSpecifierMissing(Location::new(3, 11), "assert"),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
