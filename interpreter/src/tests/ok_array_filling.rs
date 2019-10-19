//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

let original = [2, 1, 0, 3];
let mut array = [0; 4];

if 0 == original[0] {
    array[0] = 0;
};
if 1 == original[1] {
    array[1] = 1;
};
if 2 == original[2] {
    array[2] = 2;
};
if 3 == original[3] {
    array[3] = 3;
};

require(0 == array[0]);
require(1 == array[1]);
require(2 == array[2]);
require(3 == array[3]);
"#;

    let expected = Ok(());

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
