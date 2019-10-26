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

let mut sum = 0;

for i in 0..=5 {
    sum = sum + i;
    for j in 0..=5 {
        sum = sum + j;
    };
};

require(sum == 105);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
