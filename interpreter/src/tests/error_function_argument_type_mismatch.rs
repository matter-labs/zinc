//!
//! The interpreter tests.
//!

#![cfg(test)]

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

fn sum(a: u8, b: bool) -> u8 {
    let mut result = a;
    if !b {
        result = 0;
    };
    result
};

let result = sum(42, 25);

require(result == 67);
"#;

    let expected = Err(Error::FunctionArgumentTypeMismatch(
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
