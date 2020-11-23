//!
//! The structure value element tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_not_initialized() {
    let input = r#"
struct Data {}

fn main() -> Data { Data }
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_not_initialized() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() -> Data { Data }
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureNotInitialized {
        location: Location::test(6, 21),
        r#type: "Data".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_does_not_exist() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let result = Data {
        a: 0,
    }.b;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldDoesNotExist {
        location: Location::test(9, 7),
        r#type: "Data".to_owned(),
        field_name: "b".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_expected() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

fn main() {
    let result = Data {
        a: 42,
        c: 64,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldExpected {
        location: Location::test(10, 9),
        r#type: "Data".to_owned(),
        position: 2,
        expected: "b".to_owned(),
        found: "c".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_invalid_type() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let result = Data {
        a: true,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldInvalidType {
        location: Location::test(8, 9),
        r#type: "Data".to_owned(),
        field_name: "a".to_owned(),
        expected: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_count_lesser() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

fn main() {
    let result = Data {
        a: 42,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldCount {
        location: Location::test(8, 23),
        r#type: "Data".to_owned(),
        expected: 2,
        found: 1,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_count_bigger() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

fn main() {
    let result = Data {
        a: 42,
        b: 25,
        c: 64,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldCount {
        location: Location::test(11, 9),
        r#type: "Data".to_owned(),
        expected: 2,
        found: 3,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
