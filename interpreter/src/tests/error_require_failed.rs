//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Location;
use parser::Parser;

use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}

let value = 42;

require(value != 42);
"#;

    let expected = Err(Error::RequireFailed(Location::new(6, 1), "L6C1".to_owned()));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
