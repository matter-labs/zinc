//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Error as SemanticError;
use crate::semantic::Place;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let result = 42;
    result = 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::AssignmentToImmutableMemory(
        Location::new(4, 5),
        Place::new(Location::new(4, 5), "result".to_owned()),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
