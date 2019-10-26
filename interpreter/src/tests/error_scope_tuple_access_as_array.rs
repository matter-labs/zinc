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

let tuple = (1, 2, 3);
let result = tuple[1];
"#;

    let expected = Err(Error::Scope(
        Location::new(5, 14),
        ScopeError::TupleAccessAsArray("tuple".to_owned(), 1),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
