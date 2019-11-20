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

let mut value_1: field = 0;
let mut value_2: field = 1;
let mut fibo = value_1;

for i in 1..=10 {
    fibo = value_1 + value_2;
    value_1 = value_2;
    value_2 = fibo;
};

require(fibo == 89 as field);
"#;

    let expected = Ok(());

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
