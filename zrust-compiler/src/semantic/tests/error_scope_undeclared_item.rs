//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Analyzer;
use crate::semantic::Error as SemanticError;
use crate::semantic::ScopeError;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let result = undeclared(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(3, 18),
        ScopeError::UndeclaredItem("undeclared".to_owned()),
    )));

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
