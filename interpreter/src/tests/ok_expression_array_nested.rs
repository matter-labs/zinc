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

let mut array_double: [[u8; 4]; 4] = [
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12],
    [13, 14, 15, 16],
];

array_double[1][3] = 42;
array_double[2][2] = 111;
array_double[3][1] = 255;

require(array_double[1][3] == 42);
require(array_double[2][2] == 111);
require(array_double[3][1] == 255);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
