//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Parser;

use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}
witness {}
output {}

let sum = 0;

let result = sum(42, 25);

require(result == 67);
"#;

    let expected = Err(Error::CallingNotCallable("sum".to_owned()));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
