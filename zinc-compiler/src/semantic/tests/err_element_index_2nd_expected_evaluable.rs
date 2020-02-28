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
type X = field;

fn main() {
    let array = [1, 2, 3, 4, 5];
    let result = array[X];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 23),
        ElementError::OperatorIndexSecondOperandExpectedEvaluable("field".to_string()),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
