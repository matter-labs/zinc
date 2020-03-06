//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::Error as SemanticError;

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
        SemanticError::LoopBoundsExpectedConstantRangeExpression {
            location: Location::new(4, 14),
            found: Constant::Boolean(true).to_string(),
        },
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
