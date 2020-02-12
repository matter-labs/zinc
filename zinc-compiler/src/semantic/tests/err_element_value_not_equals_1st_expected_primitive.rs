//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let integer = 42;
    let value = array != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 23),
        ElementError::Value(
            ValueError::OperatorNotEqualsFirstOperandExpectedPrimitiveType(
                Type::array(Type::integer_unsigned(crate::BITLENGTH_BYTE), 3).to_string(),
            ),
        ),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
