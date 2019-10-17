//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

let mut tuple_nested: ((u8, u8), (u8, u8)) = (
    (1, 2),
    (3, 4),
);

tuple_nested.0.0 = 42;
tuple_nested.1.1 = 111;

require(tuple_nested.0.0 == 42);
require(tuple_nested.0.1 == 2);
require(tuple_nested.1.0 == 3);
require(tuple_nested.1.1 == 111);
"#;

    let expected = Ok(());

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
