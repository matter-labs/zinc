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
    a + b
};

let mut result = 0;
result = result + sum(1, 2);
result = result + sum(3, 4);
result = result + sum(5, 6);

require(result == 21);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
