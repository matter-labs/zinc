//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Error as SemanticError;
use crate::semantic::ScopeError;
use crate::semantic::Type;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let tuple = (1, 2, 3);
    let element = tuple.4;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(4, 19),
        ScopeError::TupleFieldDoesNotExist(
            4,
            Type::new_tuple(vec![
                Type::new_integer_unsigned(8),
                Type::new_integer_unsigned(8),
                Type::new_integer_unsigned(8),
            ]),
        ),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
