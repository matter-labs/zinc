//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::semantic::ValueError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let integer = 42;
    let unit = ();
    let value = unit != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedUnit(
            Type::new_integer_unsigned(8).to_string(),
        )),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
