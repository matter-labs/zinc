//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let result = (true, true, false).5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Value(ValueError::Tuple(TupleValueError::FieldDoesNotExist(
            5,
            Type::tuple(vec![Type::boolean(); 3]).to_string(),
        ))),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
