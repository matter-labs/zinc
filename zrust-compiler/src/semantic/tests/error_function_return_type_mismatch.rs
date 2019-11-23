//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Analyzer;
use crate::semantic::Error as SemanticError;
use crate::syntax::Parser;
use crate::syntax::TypeVariant;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn another() -> bool {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionReturnTypeMismatch(
        Location::new(2, 17),
        "another".to_owned(),
        TypeVariant::new_boolean(),
        TypeVariant::new_integer_unsigned(8),
    )));

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
