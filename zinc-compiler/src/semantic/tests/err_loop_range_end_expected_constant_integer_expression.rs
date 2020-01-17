//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let mut sum = 0;
    for i in 0..true {
        sum = sum + i;
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::LoopRangeEndExpectedConstantIntegerExpression(
            Location::new(4, 17),
            "true".to_owned(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
