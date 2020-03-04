//!
//! The scope tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::Error as SemanticError;

#[test]
fn error_item_is_not_namespace() {
    let input = r#"
const NOT_NAMESPACE: u8 = 42;

fn main() {
    let result = NOT_NAMESPACE::UNDEFINED;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(5, 18),
        ScopeError::ItemIsNotNamespace {
            name: "NOT_NAMESPACE".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_redeclared() {
    let input = r#"
fn main() {
    let result = 42;
    {
        let result = 69;
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(5, 9),
        ScopeError::ItemRedeclared {
            name: "result".to_owned(),
            reference: Some(Location::new(3, 9)),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared() {
    let input = r#"
fn main() {
    result = 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(3, 5),
        ScopeError::ItemUndeclared {
            name: "result".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_enum_variant() {
    let input = r#"
enum Jabberwocky {
    Gone = 42,
}

fn main() {
    let really = Jabberwocky::Exists;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        Location::new(7, 31),
        ScopeError::ItemUndeclared {
            name: "Exists".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
