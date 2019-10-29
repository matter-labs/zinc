//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Location;
use parser::Parser;
use parser::TypeVariant;

use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}
witness {}
output {}

let value: u8 = true;
"#;

    let expected = Err(Error::LetInvalidType(
        Location::new(6, 12),
        TypeVariant::new_boolean(),
        TypeVariant::new_integer_unsigned(8),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
