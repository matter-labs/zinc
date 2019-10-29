//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Parser;

use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}
witness {}
output {}

fn sum(a: u8, b: u8) -> u8 {
    let result = a + b;
    result
};

let result = sum(42);

require(result == 67);
"#;

    let expected = Err(Error::MissingFunctionArgument("b".to_owned()));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
