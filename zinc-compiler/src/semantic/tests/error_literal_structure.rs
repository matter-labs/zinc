//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Error as SemanticError;
use crate::semantic::StructureError;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let result = struct Data {
        a: 0,
        a: 1,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::LiteralStructure(
        Location::new(9, 9),
        StructureError::FieldAlreadyExists("a".to_owned()),
    )));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
