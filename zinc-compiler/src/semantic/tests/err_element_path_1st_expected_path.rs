//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = 5::UNDEFINED;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 18),
        ElementError::OperatorPathFirstOperandExpectedPath("5: u8".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
