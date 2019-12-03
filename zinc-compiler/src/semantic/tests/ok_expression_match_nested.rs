//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use parser::Parser;

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}
witness {}
output {}

let value = 2;
let result = match value {
    1 => 1,
    inner => match inner {
        255 => 0,
        _ => 42,
    },
};

require(result == 42);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
