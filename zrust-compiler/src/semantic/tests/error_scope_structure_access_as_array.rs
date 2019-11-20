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

let result = test[1];
"#;

    let expected = Err(Error::Scope(
        Location::new(9, 14),
        ScopeError::StructureAccessAsArray("test".to_owned(), 1),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
