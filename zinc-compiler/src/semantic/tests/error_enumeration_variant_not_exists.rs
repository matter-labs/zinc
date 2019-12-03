//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use parser::Location;
use parser::Parser;

use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}
witness {}
output {}

enum Jabberwocky {
    Gone = 42,
};

let really = Jabberwocky::Exists;
"#;

    let expected = Err(Error::EnumerationVariantNotExists(
        Location::new(10, 25),
        "Jabberwocky".to_owned(),
        "Exists".to_owned(),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
