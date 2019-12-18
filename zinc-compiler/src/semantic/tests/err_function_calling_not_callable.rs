//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
type another = (u8, u8);

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionCallingNotCallableObject(
            Location::new(5, 24),
            "(u8, u8)".to_owned(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
