//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn another() -> bool {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionReturnTypeMismatch(
        Location::new(2, 17),
        "another".to_owned(),
        Type::boolean().to_string(),
        Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
