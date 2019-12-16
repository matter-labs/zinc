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
const NOT_TYPE: u8 = 69;

fn main() {
    let value = 42 as NOT_TYPE;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeAliasDoesNotPointToType(
        Location::new(5, 23),
        "NOT_TYPE".to_owned(),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
