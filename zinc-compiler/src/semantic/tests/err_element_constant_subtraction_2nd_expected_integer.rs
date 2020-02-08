//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = 42 - true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorSubtractionSecondOperandExpectedInteger(Constant::Boolean(true)),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
