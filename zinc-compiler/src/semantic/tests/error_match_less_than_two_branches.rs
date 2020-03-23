//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::Error as SemanticError;

#[test]
fn test() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        _ => 10,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MatchLessThanTwoBranches {
        location: Location::new(4, 18),
    }));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
