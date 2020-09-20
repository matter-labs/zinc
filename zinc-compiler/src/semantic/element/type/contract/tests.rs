//!
//! The semantic analyzer contract type element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::error::Error as ScopeError;

#[test]
fn error_duplicate_field() {
    let input = r#"
contract Contract {
    a: u8;
    b: u8;
    b: field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemRedeclared {
            location: Location::test(5, 5),
            name: "b".to_owned(),
            reference: Some(Location::test(4, 5)),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
