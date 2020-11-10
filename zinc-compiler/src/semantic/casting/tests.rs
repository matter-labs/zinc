//!
//! The type caster tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_integer_lesser_bitlength_same_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as u32;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_lesser_bitlength_different_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as i32;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_equal_bitlength_same_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as u64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_equal_bitlength_different_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as i64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_greater_bitlength_same_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as u128;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_greater_bitlength_different_sign() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as i128;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_unsigned_to_field() {
    let input = r#"
fn main() {
    let value: u64 = 0;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_signed_to_field() {
    let input = r#"
fn main() {
    let value: i64 = 0;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_enumeration_to_unsigned() {
    let input = r#"
enum List {
    VALUE = 42,
}

fn main() {
    let value = List::VALUE;
    let result = value as u64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_enumeration_to_signed() {
    let input = r#"
enum List {
    VALUE = 42,
}

fn main() {
    let value = List::VALUE;
    let result = value as i64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_enumeration_to_field() {
    let input = r#"
enum List {
    VALUE = 42,
}

fn main() {
    let value = List::VALUE;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer_field_to_field() {
    let input = r#"
fn main() {
    let value: field = 0;
    let result = value as field;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_same_type() {
    let input = r#"
struct Data {
    value: u8,
}

fn main() {
    let data = Data { value: 42 };
    let result = data as Data;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_casting_from_invalid_type() {
    let input = r#"
fn main() {
    let value: field = 0;
    let result = value as u8;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorCastingTypesMismatch {
            location: Location::test(4, 18),
            inner: CastingError::CastingFromInvalidType {
                from: Type::field(None).to_string(),
                to: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
            reference: Location::test(4, 27),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_casting_to_invalid_type() {
    let input = r#"
fn main() {
    let value: u8 = 0;
    let result = value as bool;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorCastingTypesMismatch {
            location: Location::test(4, 18),
            inner: CastingError::CastingToInvalidType {
                from: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
                to: Type::boolean(None).to_string(),
            },
            reference: Location::test(4, 27),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_casting_to_invalid_type_let_implicit() {
    let input = r#"
fn main() {
    let value = 42;
    let result: bool = value;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorCastingTypesMismatch {
            location: Location::test(4, 24),
            inner: CastingError::CastingToInvalidType {
                from: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
                to: Type::boolean(None).to_string(),
            },
            reference: Location::test(4, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
