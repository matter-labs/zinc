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
fn unknown() {}

fn main(input: (), witness: ()) {
    unknown!();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionNotInstruction(
        Location::new(5, 13),
        "unknown".to_owned(),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
