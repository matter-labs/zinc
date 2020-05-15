//!
//! The tuple value element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_field_does_not_exist() {
    let input = r#"
fn main() {
    let result = (true, true, false).5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::Tuple(TupleValueError::FieldDoesNotExist {
            location: Location::new(3, 38),
            type_identifier: Type::tuple(Some(Location::new(3, 38)), vec![Type::boolean(None); 3])
                .to_string(),
            field_index: 5,
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
