//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Parser;

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}

fn sum(a: u8, b: u8) -> u8 {
    let result = a + b;
    result
};

let result = sum(42, 25);

require(result == 67);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
