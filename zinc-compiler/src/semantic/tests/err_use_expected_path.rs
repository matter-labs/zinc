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
use 5;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::UseExpectedPath {
        location: Location::new(2, 5),
        found: "constant integer '5' of type 'u8'".to_owned(),
    }));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
