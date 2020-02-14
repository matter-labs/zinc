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
    let boolean = true;
    let integer = 42;
    let value = integer || boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorOrFirstOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}