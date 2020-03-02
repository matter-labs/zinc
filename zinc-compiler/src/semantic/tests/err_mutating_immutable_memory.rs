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
    let result = 42;
    result = 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MutatingImmutableMemory(
        Location::new(4, 12),
        "result".to_string(),
        Some(Location::new(3, 9)),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
