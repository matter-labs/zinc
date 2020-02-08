//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
enum Value {
    FIRST = 1,
}

fn main() {
    let value = Value::5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 22),
        ElementError::OperatorPathSecondOperandExpectedMemberString("5: u8".to_owned()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
