//!
//! The interpreter tests.
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

let result = 2 + 2 * 2 - (42 - 7 * 3) == 6 - 21 && (false ^^ (true || (2 + 2 == 5)));

require(result);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
