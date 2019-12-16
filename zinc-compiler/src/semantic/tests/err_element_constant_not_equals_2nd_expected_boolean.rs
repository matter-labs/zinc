//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::Constant;
use crate::semantic::ConstantError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::IntegerConstant;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = true != 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorNotEqualsSecondOperandExpectedBoolean(Constant::Integer(
                IntegerConstant::from((42, 8)),
            )),
        ),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
