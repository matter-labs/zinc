//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use std::convert::TryFrom;

use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let variable = 42;
    const CONSTANT: u8 = variable;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ConstantExpressionHasNonConstantElement(
            Location::new(4, 26),
            Element::Value(Value::try_from(Type::integer_unsigned(crate::BITLENGTH_BYTE)).unwrap())
                .to_string(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
