//!
//! A semantic analyzer test.
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

let mut array = [0; 4];
array[0] = 0;
array[1] = 1;
array[2] = 2;
array[3] = 3;

require(0 == array[0]);
require(1 == array[1]);
require(2 == array[2]);
require(3 == array[3]);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
