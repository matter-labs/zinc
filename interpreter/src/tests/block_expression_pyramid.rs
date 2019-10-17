//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

let pyramid = 1 + {
    2 + {
        3 + {
            4
        } + 3
    } + 2
} + 1;

require(pyramid == 16);
"#;

    let expected = Ok(());

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
