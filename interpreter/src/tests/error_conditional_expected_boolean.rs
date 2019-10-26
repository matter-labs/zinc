//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Location;
use parser::Parser;
use r1cs::TestConstraintSystem;

use crate::Error;
use crate::Interpreter;
use crate::Value;

#[test]
fn test() {
    let input = r#"
input {}

if 42 {};
"#;

    let expected = Err(Error::ConditionalExpectedBooleanExpression(
        Location::new(4, 4),
        Value::new_integer_from_usize(TestConstraintSystem::new(), 42, 8).expect("Always valid"),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
