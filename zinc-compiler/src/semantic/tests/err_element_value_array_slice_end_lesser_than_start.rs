//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    [1, 2, 3, 4, 5][2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Value(ValueError::Array(ArrayValueError::SliceEndLesserThanStart(
            "1".to_owned(),
            "2".to_owned(),
        ))),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
