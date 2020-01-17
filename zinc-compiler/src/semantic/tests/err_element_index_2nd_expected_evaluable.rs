//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
type X = field;

fn main(input: (), witness: ()) {
    let array = [1, 2, 3, 4, 5];
    let result = array[X];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 23),
        ElementError::OperatorIndexSecondOperandExpectedEvaluable("field".to_string()),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
