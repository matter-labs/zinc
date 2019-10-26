//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Location;
use parser::Parser;

use crate::scope::Error as ScopeError;
use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}

let array = [1, 2, 3];
let result = array[69];
"#;

    let expected = Err(Error::Scope(
        Location::new(5, 14),
        ScopeError::ArrayIndexOutOfRange(69, "array".to_owned()),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
