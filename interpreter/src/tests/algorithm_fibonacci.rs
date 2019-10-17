//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

let mut value_1: field = 0;
let mut value_2: field = 1;
let mut fibo = value_1;

for i in 1..=6 {
    fibo = value_1 + value_2;
    value_1 = value_2;
    value_2 = fibo;
};

require(fibo == 13 as field);
"#;

    let expected = Ok(());

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
