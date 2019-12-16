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
    let scrutinee = 42;
    let result = match scrutinee {
        1 => 10,
        2 => 20,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MatchNotExhausted(
        Location::new(4, 18),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
