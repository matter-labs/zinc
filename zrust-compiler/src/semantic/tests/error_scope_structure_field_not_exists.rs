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

struct Test { value: u8 };
let test = struct Test { value: 42 };

let result = test.jabberwocky;
"#;

    let expected = Err(Error::Scope(
        Location::new(9, 14),
        ScopeError::StructureFieldNotExists("jabberwocky".to_owned(), "test".to_owned()),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
