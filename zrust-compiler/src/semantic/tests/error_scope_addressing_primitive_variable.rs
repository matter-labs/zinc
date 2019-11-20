//!
//! A semantic analyzer test.
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
witness {}
output {}

let not_array = 42;
let result = not_array[69];
"#;

    let expected = Err(Error::Scope(
        Location::new(7, 14),
        ScopeError::AddressingPrimitiveTypeVariable("not_array".to_owned()),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
