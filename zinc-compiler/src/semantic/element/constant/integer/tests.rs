//!
//! The integer constant element tests.
//!

#![cfg(test)]

use std::str::FromStr;

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::integer::Integer;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_minimal_bitlength() {
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("0").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("255").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("256").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("65535").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("65536").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 3),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("4294967295").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 4),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("4294967296").unwrap_or_default(), false),
        Ok(crate::BITLENGTH_BYTE * 5),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("18446744073709551615").unwrap_or_default(),
            false
        ),
        Ok(crate::BITLENGTH_BYTE * 8),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("18446744073709551616").unwrap_or_default(),
            false
        ),
        Ok(crate::BITLENGTH_BYTE * 9),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("-128").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("127").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 1),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("128").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("32767").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 2),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("32768").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 3),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("2147483647").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 4),
    );
    assert_eq!(
        Integer::minimal_bitlength(&BigInt::from_str("2147483648").unwrap_or_default(), true),
        Ok(crate::BITLENGTH_BYTE * 5),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("9223372036854775807").unwrap_or_default(),
            true
        ),
        Ok(crate::BITLENGTH_BYTE * 8),
    );
    assert_eq!(
        Integer::minimal_bitlength(
            &BigInt::from_str("9223372036854775808").unwrap_or_default(),
            true
        ),
        Ok(crate::BITLENGTH_BYTE * 9),
    );
}

#[test]
fn error_integer_too_large_ordinar_constant() {
    let input = r#"
fn main() {
    let invalid = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 19),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(crate::semantic::tests::PANIC_TEST_DATA),
                bitlength: crate::BITLENGTH_FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_integer_too_large_loop_for_bound() {
    let input = r#"
fn main() {
    for i in 0..0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff {}
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(crate::semantic::tests::PANIC_TEST_DATA),
                bitlength: crate::BITLENGTH_FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_integer_too_large_pattern_match() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff => 10,
        2 => 20,
        _ => 30,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 9),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(crate::semantic::tests::PANIC_TEST_DATA),
                bitlength: crate::BITLENGTH_FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_equals() {
    let input = r#"
fn main() {
    let value = 42 as u64 >= 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_equals_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value >= 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_equals_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value >= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_equals() {
    let input = r#"
fn main() {
    let value = 42 as u64 <= 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_equals_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value <= 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_equals_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value <= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater() {
    let input = r#"
fn main() {
    let value = 42 as u64 > 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value > 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value > Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser() {
    let input = r#"
fn main() {
    let value = 42 as u64 < 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value < 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value < Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_or() {
    let input = r#"
fn main() {
    let value = 42 as u64 | 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_or_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value | 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_or_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value | Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_xor() {
    let input = r#"
fn main() {
    let value = 42 as u64 ^ 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_xor_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value ^ 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_xor_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value ^ Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_and() {
    let input = r#"
fn main() {
    let value = 42 as u64 & 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_and_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value & 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_and_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value & Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_addition() {
    let input = r#"
fn main() {
    let value = 42 as u64 + 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_addition_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value + 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_addition_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value + Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_subtraction() {
    let input = r#"
fn main() {
    let value = 42 as u64 - 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_subtraction_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value - 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_subtraction_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value - Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_multiplication() {
    let input = r#"
fn main() {
    let value = 42 as u64 * 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_multiplication_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value * 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_multiplication_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value * Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_division() {
    let input = r#"
fn main() {
    let value = 42 as u64 / 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_division_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value / 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_division_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value / Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_remainder() {
    let input = r#"
fn main() {
    let value = 42 as u64 % 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_remainder_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value % 69;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 32),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                first: "enum Default".to_owned(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_remainder_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 69,
}

fn main() {
    let value = One::Value % Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(11, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                first: "enum One".to_owned(),
                second: "enum Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_addition_signed_negative() {
    let input = r#"
fn main() {
    let value = -120 + (-50);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                value: BigInt::from(-170),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_addition_signed_positive() {
    let input = r#"
fn main() {
    let value = 42 as i8 + 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                value: BigInt::from(142),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_addition_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 + 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                value: BigInt::from(297),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_subtraction_signed_negative() {
    let input = r#"
fn main() {
    let value = -42 - 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                value: BigInt::from(-142),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_subtraction_signed_positive() {
    let input = r#"
fn main() {
    let value = (50 as i8) - (-100);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                value: BigInt::from(150),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_subtraction_unsigned_negative() {
    let input = r#"
fn main() {
    let value = 42 - 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                value: BigInt::from(-213),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_multiplication_signed_negative() {
    let input = r#"
fn main() {
    let value = -100 * (2 as i8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                value: BigInt::from(-200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_multiplication_signed_positive() {
    let input = r#"
fn main() {
    let value = 100 as i8 * 2 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                value: BigInt::from(200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_multiplication_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 * 10;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                value: BigInt::from(420),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_division_signed_positive() {
    let input = r#"
fn main() {
    let value = -128 / (-1);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowDivision {
                value: BigInt::from(128),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_casting_signed_positive() {
    let input = r#"
fn main() {
    let value = 200 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                value: BigInt::from(200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_casting_unsigned_negative() {
    let input = r#"
fn main() {
    let value = (-100 as u8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 23),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                value: BigInt::from(-100),
                r#type: Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_negation_signed_positive() {
    let input = r#"
fn main() {
    let value = --128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                value: BigInt::from(128),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_negation_unsigned_negative() {
    let input = r#"
fn main() {
    let value = -200;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                value: BigInt::from(-200),
                r#type: Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_division() {
    let input = r#"
fn main() {
    let value = 42 as field / 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 29),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldDivision,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_remainder() {
    let input = r#"
fn main() {
    let value = 42 as field % 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 29),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldRemainder,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_or() {
    let input = r#"
fn main() {
    let value = 42 as field | 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 29),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_xor() {
    let input = r#"
fn main() {
    let value = 42 as field ^ 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 29),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_and() {
    let input = r#"
fn main() {
    let value = 42 as field & 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 29),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_shift_left() {
    let input = r#"
fn main() {
    let value = 42 as field << 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 29),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_shift_right() {
    let input = r#"
fn main() {
    let value = 42 as field >> 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 29),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_not() {
    let input = r#"
fn main() {
    let value = ~(42 as field);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_negation() {
    let input = r#"
fn main() {
    let value = -(42 as field);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldNegation,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_zero_division() {
    let input = r#"
fn main() {
    let value = 42 / 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroDivision)),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_zero_remainder() {
    let input = r#"
fn main() {
    let value = 42 % 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroRemainder)),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
