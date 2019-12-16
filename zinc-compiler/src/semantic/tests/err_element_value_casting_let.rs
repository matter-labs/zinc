//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::CasterError;
use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::semantic::ValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = 42;
    let result: bool = value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 17),
        ElementError::Value(ValueError::Casting(CasterError::ToInvalidType(
            Type::new_integer_unsigned(8).to_string(),
            Type::new_boolean().to_string(),
        ))),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
