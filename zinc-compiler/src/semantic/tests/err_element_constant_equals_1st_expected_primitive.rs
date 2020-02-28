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
    let value = "string" == 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Constant(
            ConstantError::OperatorEqualsFirstOperandExpectedPrimitiveType(
                Constant::String("string".to_owned()).to_string(),
            ),
        ),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
