//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Location;
use parser::Parser;
use parser::TypeVariant;
use semantic::CastingError;

use crate::element::IntegerError;
use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}

let value: bool = 42;
"#;

    let expected = Err(Error::LetImplicitCasting(
        Location::new(4, 12),
        IntegerError::Casting(CastingError::ToInvalidType(
            TypeVariant::new_integer_unsigned(8),
            TypeVariant::new_boolean(),
        )),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
