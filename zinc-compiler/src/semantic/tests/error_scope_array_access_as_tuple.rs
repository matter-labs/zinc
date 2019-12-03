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

let array = [1, 2, 3];
let result = array.69;
"#;

    let expected = Err(Error::Scope(
        Location::new(7, 14),
        ScopeError::ArrayAccessAsTuple("array".to_owned(), 69),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
