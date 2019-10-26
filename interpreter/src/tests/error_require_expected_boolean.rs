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

require(42);
"#;

    let expected = Err(Error::RequireExpectedBooleanExpression(
        Location::new(4, 9),
        "L4C1".to_owned(),
        Value::new_integer_from_usize(TestConstraintSystem::new(), 42, 8).expect("Always valid"),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
