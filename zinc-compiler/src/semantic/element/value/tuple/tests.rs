//!
//! The tuple value element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;
use crate::semantic::Error as SemanticError;

#[test]
fn error_field_does_not_exist() {
    let input = r#"
fn main() {
    let result = (true, true, false).5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Value(ValueError::Tuple(TupleValueError::FieldDoesNotExist {
            type_identifier: Type::tuple(vec![Type::boolean(); 3]).to_string(),
            field_index: 5,
        })),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
