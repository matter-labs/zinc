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

    let expected = Err(Error::Semantic(SemanticError::UseStatementExpectedPlace(
        Location::new(2, 5),
        "5: u8".to_owned(),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
