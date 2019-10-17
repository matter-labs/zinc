//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

struct Test {
    x: u8,
    y: u8,
    z: u8,
};

let mut test = Test {
    x: 1,
    y: 2,
    z: 3,
};

test.x = 5;
test.y = 7;
test.z = 9;
require(test.x == 5);
require(test.y == 7);
require(test.z == 9);
"#;

    let expected = Ok(());

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
