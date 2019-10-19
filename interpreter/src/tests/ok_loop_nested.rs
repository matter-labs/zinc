//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

let mut sum: u64 = 0;

for i in 0..=5 {
    sum = sum + i;
    for j in 0..=5 {
        sum = sum + j;
    };
};

require(sum == 105 as u64);
"#;

    let expected = Ok(());

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
