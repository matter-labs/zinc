//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::semantic::ValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let integer = 42;
    let value = array == integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 23),
        ElementError::Value(ValueError::OperatorEqualsFirstOperandExpectedPrimitiveType(
            Type::new_array(Type::new_integer_unsigned(crate::BITLENGTH_BYTE), 3).to_string(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
