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

let result = 42;
result = 69;
"#;

    let expected = Err(Error::Scope(
        Location::new(5, 8),
        ScopeError::MutatingImmutable("result".to_owned()),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
