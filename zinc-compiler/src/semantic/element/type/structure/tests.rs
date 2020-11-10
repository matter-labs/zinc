//!
//! The semantic analyzer structure type element tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_duplicate_field() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
    b: field,
}

fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeDuplicateField {
        location: Location::test(5, 5),
        r#type: "Data".to_owned(),
        field_name: "b".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_generics() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeExpectedGenerics {
        location: Location::test(5, 13),
        r#type: "MTreeMap".to_owned(),
        expected: 2,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_unexpected_generics() {
    let input = r#"
struct Unexpected {
    x: field,
}

struct Data {
    a: u8,
    b: u8,
    c: Unexpected<bool>,
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeUnexpectedGenerics {
        location: Location::test(9, 8),
        r#type: "Unexpected".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_invalid_generics_number() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248, bool>;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeInvalidGenericsNumber {
        location: Location::test(5, 13),
        r#type: "MTreeMap".to_owned(),
        expected: 2,
        found: 3,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
