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

let result = 42 + jabberwocky;
"#;

    let expected = Err(Error::Scope(
        Location::new(4, 17),
        ScopeError::UndeclaredItem("jabberwocky".to_owned()),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
