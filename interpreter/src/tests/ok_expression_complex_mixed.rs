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

struct Data { value: u8 };

let array = [[1, 2], [3, 4]];
let tuple = ((1, 2), (3, 4));
let structure = struct Data { value: 34 };

let result = array[1][1] + tuple.1.1 + structure.value;

require(result == 42);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
