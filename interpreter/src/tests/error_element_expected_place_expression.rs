//!
//! The interpreter tests.
//!

#![cfg(test)]

use parser::Location;
use parser::Parser;
use r1cs::TestConstraintSystem;

use crate::element::Element;
use crate::element::Error as ElementError;
use crate::element::Value;
use crate::Error;
use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
input {}

5 = 5;
"#;

    let expected = Err(Error::Element(
        Location::new(4, 3),
        ElementError::ExpectedPlaceExpression(
            "assign",
            Element::Value(
                Value::new_integer_from_usize(TestConstraintSystem::new(), 5, 8)
                    .expect("Always valid"),
            ),
        ),
    ));

    let result = Interpreter::default().interpret(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
