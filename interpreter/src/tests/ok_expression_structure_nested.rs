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

struct Test {
    inner: Inner,
};

struct Inner {
    value: u8,
};

let test = struct Test {
    inner: struct Inner {
        value: 3,
    },
};

require(test.inner.value == 3);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
