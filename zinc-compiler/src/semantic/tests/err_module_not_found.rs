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
mod unknown;

fn main(input: (), witness: ()) {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ModuleNotFound(
        Location::new(2, 5),
        "unknown".to_owned(),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
