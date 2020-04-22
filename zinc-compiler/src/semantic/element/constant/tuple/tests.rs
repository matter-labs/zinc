//!
//! The constant tuple element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::tuple::error::Error as TupleConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_field_does_not_exist() {
    let input = r#"
fn main() {
    const VALUE: bool = (true, true, false).5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 44),
        ElementError::Constant(ConstantError::Tuple(
            TupleConstantError::FieldDoesNotExist {
                type_identifier: Type::tuple(vec![Type::boolean(); 3]).to_string(),
                field_index: 5,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
