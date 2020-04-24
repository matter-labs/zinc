//!
//! The scope tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::error::Error as ScopeError;

#[test]
fn ok_current_scope() {
    let input = r#"
fn main() {
    const VALUE: u8 = 42;

    let result = VALUE;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_upper_scope() {
    let input = r#"
const VALUE: u8 = 42;

fn main() {
    let result = VALUE;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_far_scope() {
    let input = r#"
const VALUE: u8 = 42;

fn main() {
    {
        {
            {
                {
                    let result = VALUE;
                }
            }
        }
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_item_is_not_namespace() {
    let input = r#"
const NOT_NAMESPACE: u8 = 42;

fn main() {
    let result = NOT_NAMESPACE::UNDEFINED;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemNotNamespace {
            location: Location::new(5, 18),
            name: "NOT_NAMESPACE".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_redeclared() {
    let input = r#"
fn main() {
    let result = 42;
    {
        let result = 64;
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemRedeclared {
            location: Location::new(5, 13),
            name: "result".to_owned(),
            reference: Some(Location::new(3, 9)),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared() {
    let input = r#"
fn main() {
    result = 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemUndeclared {
            location: Location::new(3, 5),
            name: "result".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_lower() {
    let input = r#"
fn main() {
    {
        let result = 42;
    };
    result = 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemUndeclared {
            location: Location::new(6, 5),
            name: "result".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

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
        ScopeError::ItemUndeclared {
            location: Location::new(7, 31),
            name: "Exists".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_enum_variant_outside() {
    let input = r#"
const Gone: u8 = 42;

enum Jabberwocky {}

fn main() {
    let really = Jabberwocky::Exists;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemUndeclared {
            location: Location::new(7, 31),
            name: "Exists".to_owned(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_self_lowercase() {
    let input = r#"
fn not_method(self) -> bool {
    42
}

fn main() {
    let value = not_method();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemUndeclared {
            location: Location::new(2, 15),
            name: Keyword::SelfUppercase.to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_self_uppercase() {
    let input = r#"
fn not_method(value: Self) -> bool {
    42
}

fn main() {
    let value = not_method();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ItemUndeclared {
            location: Location::new(2, 22),
            name: Keyword::SelfUppercase.to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_redeclared() {
    let input = r#"
contract Uniswap {
    pub fn deposit(amount: u248) -> bool { true }
}

contract Multiswap {
    pub fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Scope(
        ScopeError::ContractRedeclared {
            location: Location::new(6, 10),
            reference: Location::new(2, 10),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
