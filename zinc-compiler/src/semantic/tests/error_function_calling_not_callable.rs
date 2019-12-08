//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::BinaryAnalyzer;
use crate::semantic::Element;
use crate::semantic::Error as SemanticError;
use crate::semantic::Place;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let another = false;
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionCallNotCallableObject(
            Location::new(4, 24),
            Element::Place(Place::new(Location::new(4, 17), "another".to_owned())),
        ),
    ));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
