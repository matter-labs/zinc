//!
//! The contract value element tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_field_does_not_exist() {
    let input = r#"
contract Test {
    a: u8;
    b: u8;

    fn main() -> u8 { Self { a: 5, b: 10 }.c }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldDoesNotExist {
        location: Location::test(6, 44),
        r#type: "Test".to_owned(),
        field_name: "c".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_expected() {
    let input = r#"
contract Test {
    a: u8;
    b: u8;

    fn main() -> Self { Self { a: 5, c: 10 } }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldExpected {
        location: Location::test(6, 38),
        r#type: "Test".to_owned(),
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
contract Test {
    a: u8;
    b: u8;

    fn main() -> Self { Self { a: 5, b: true } }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldInvalidType {
        location: Location::test(6, 38),
        r#type: "Test".to_owned(),
        field_name: "b".to_owned(),
        expected: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_count_lesser() {
    let input = r#"
contract Test {
    a: u8;
    b: u8;

    fn main() -> Self { Self { a: 5 } }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldCount {
        location: Location::test(6, 30),
        r#type: "Test".to_owned(),
        expected: 2,
        found: 1,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_field_count_bigger() {
    let input = r#"
contract Test {
    a: u8;
    b: u8;

    fn main() -> Self { Self { a: 5, b: 10, c: 15 } }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldCount {
        location: Location::test(6, 45),
        r#type: "Test".to_owned(),
        expected: 2,
        found: 3,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
