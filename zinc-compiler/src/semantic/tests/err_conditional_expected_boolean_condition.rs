//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::Error as SemanticError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    if 42 { 1 } else { 2 }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ConditionalExpectedBooleanCondition(
            Location::new(3, 8),
            Type::new_integer_unsigned(8).to_string(),
        ),
    ));

    let result = super::result(input);

    assert_eq!(expected, result);
}
