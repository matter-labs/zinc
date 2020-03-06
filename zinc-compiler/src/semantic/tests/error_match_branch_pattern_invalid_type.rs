//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

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
        SemanticError::MatchBranchPatternInvalidType {
            location: Location::new(5, 9),
            expected: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            found: Type::boolean().to_string(),
            reference: Location::new(4, 24),
        },
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
