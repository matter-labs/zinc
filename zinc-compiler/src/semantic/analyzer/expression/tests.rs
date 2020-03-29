//!
//! The expression tests.
//!

#![cfg(test)]

use std::convert::TryFrom;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::Error as SemanticError;

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

#[test]
fn error_mutating_immutable_memory() {
    let input = r#"
fn main() {
    let result = 42;
    result = 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MutatingImmutableMemory {
        location: Location::new(4, 12),
        name: "result".to_string(),
        reference: Some(Location::new(3, 9)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_mutating_with_different_type() {
    let input = r#"
fn main() {
    let mut result = 42;
    result = false;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MutatingWithDifferentType {
        location: Location::new(4, 12),
        expected: Type::boolean().to_string(),
        found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
