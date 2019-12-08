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
struct Data {
    a: u8,
}

fn main() {
    let data = struct Data {
        a: 1,
    };
    let element = data.b;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(10, 19),
        ScopeError::StructureFieldDoesNotExist(
            "b".to_owned(),
            Type::new_structure(
                "Data".to_owned(),
                vec![("a".to_owned(), Type::new_integer_unsigned(8))],
            ),
        ),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
