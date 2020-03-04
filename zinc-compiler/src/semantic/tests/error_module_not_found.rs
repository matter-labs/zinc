//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::Error as SemanticError;

#[test]
fn test() {
    let input = r#"
mod unknown;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ModuleNotFound {
        location: Location::new(2, 5),
        name: "unknown".to_owned(),
    }));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
