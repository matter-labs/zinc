//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Location;

use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

let value = 42;

require(value != 42);
"#;

    let expected = Err(Error::RequireFailed(Location::new(6, 1), "L6".to_owned()));

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
