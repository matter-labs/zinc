//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::semantic::Type;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main(input: (), witness: ()) {
    let value = another(false);
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionArgumentTypeMismatch(
            Location::new(7, 24),
            "another".to_owned(),
            "x".to_owned(),
            Type::new_integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            Type::new_boolean().to_string(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
