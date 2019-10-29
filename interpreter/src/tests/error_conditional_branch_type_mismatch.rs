//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::BooleanLiteral;
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
witness {}
output {}

if true { 42 } else { false };
"#;

    let expected = Err(Error::ConditionalBranchTypeMismatch(
        Location::new(6, 1),
        Value::new_integer_from_usize(TestConstraintSystem::new(), 42, 8).expect("Always valid"),
        Value::new_boolean_from_literal(TestConstraintSystem::new(), BooleanLiteral::False)
            .expect("Always valid"),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
