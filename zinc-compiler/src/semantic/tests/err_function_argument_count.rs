//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Function(
        Location::new(7, 24),
        FunctionError::ArgumentCount("another".to_owned(), 1, 0),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
