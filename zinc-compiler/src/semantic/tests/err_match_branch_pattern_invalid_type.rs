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
        false => 0,
        true => 1,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchPatternInvalidType(
            Location::new(5, 9),
            Type::new_boolean().to_string(),
            Type::new_integer_unsigned(8).to_string(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
