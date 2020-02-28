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
fn main() {
    let mut sum = 0;
    for i in true {
        sum = sum + i;
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::LoopBoundsExpectedConstantRangeExpression(
            Location::new(4, 14),
            "true".to_owned(),
        ),
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
