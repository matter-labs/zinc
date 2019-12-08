//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Error as SemanticError;
use crate::semantic::ScopeError;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
enum Jabberwocky {
    Gone = 42,
}

fn main() {
    let really = Jabberwocky::Exists;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(7, 29),
        ScopeError::EnumerationVariantDoesNotExist("Exists".to_owned(), "Jabberwocky".to_owned()),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
