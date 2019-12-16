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
    let scrutinee = 42;
    let result = match scrutinee {
        0 => false,
        1 => 0,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchExpressionInvalidType(
            Location::new(6, 14),
            Type::new_integer_unsigned(8).to_string(),
            Type::new_boolean().to_string(),
        ),
    ));

    let result = super::result(input);

    assert_eq!(expected, result);
}
