//!
//! The expression tests.
//!

#![cfg(test)]

use std::convert::TryFrom;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_const_expression_has_non_const_element() {
    let input = r#"
fn main() {
    let variable = 42;
    const CONSTANT: u8 = variable;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ConstantExpressionHasNonConstantElement {
            location: Location::new(4, 26),
            found: Element::Value(
                Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                    .expect(crate::semantic::tests::PANIC_TEST_DATA),
            )
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
