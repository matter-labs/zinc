//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::TupleValueError;
use crate::semantic::Type;
use crate::semantic::ValueError;

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
            Type::new_tuple(vec![Type::new_boolean(); 3]).to_string(),
        ))),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
