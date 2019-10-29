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
witness {}
output {}

let mut sum: u24 = 0;
for i in 0..1_000_000 while i < 10 as u24 {
    sum = sum + i;
};

require(sum == 55 as u24);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
