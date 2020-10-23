//!
//! The constant integer element tests.
//!

use std::str::FromStr;

use num::BigInt;

use zinc_math::InferenceError;

use crate::error::Error;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use zinc_lexical::Location;

#[test]
fn error_types_mismatch_greater_equals() {
    let input = r#"
fn main() {
    let value = 42 as u64 >= 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value >= 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

fn main() {
    let value = One::Value >= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 <= 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value <= 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

fn main() {
    let value = One::Value <= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 > 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value > 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

fn main() {
    let value = One::Value > Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 < 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value < 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

fn main() {
    let value = One::Value < Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitor() {
    let input = r#"
fn main() {
    let value = 42 as u64 | 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitor_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value | 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitor_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value | Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitxor() {
    let input = r#"
fn main() {
    let value = 42 as u64 ^ 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitxor_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value ^ 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitxor_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value ^ Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitand() {
    let input = r#"
fn main() {
    let value = 42 as u64 & 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitand_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value & 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitand_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value & Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 + 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value + 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

fn main() {
    let value = One::Value + Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 - 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value - 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

enum Two {
    Value = 42,
}

fn main() {
    let value = One::Value - Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 * 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value * 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 5,
}

fn main() {
    let value = One::Value * Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 / 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value / 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

fn main() {
    let value = One::Value / Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 % 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                location: Location::test(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
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
    let value = Default::Value % 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                location: Location::test(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
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
    Value = 64,
}

fn main() {
    let value = One::Value % Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                location: Location::test(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_left_2nd_operand_expected_unsigned() {
    let input = r#"
fn main() {
    let value = 168 << -2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
                location: Location::test(3, 25),
                found: IntegerConstant::new(
                    Location::test(3, 25),
                    BigInt::from(-2),
                    true,
                    zinc_const::bitlength::BYTE,
                    true,
                )
                .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_right_2nd_operand_expected_unsigned() {
    let input = r#"
fn main() {
    let value = 42 >> -2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                location: Location::test(3, 24),
                found: IntegerConstant::new(
                    Location::test(3, 24),
                    BigInt::from(-2),
                    true,
                    zinc_const::bitlength::BYTE,
                    true,
                )
                .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                location: Location::test(3, 18),
                value: BigInt::from(-170),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                location: Location::test(3, 17),
                value: BigInt::from(142),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                location: Location::test(3, 17),
                value: BigInt::from(297),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                location: Location::test(3, 18),
                value: BigInt::from(-142),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                location: Location::test(3, 18),
                value: BigInt::from(150),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                location: Location::test(3, 17),
                value: BigInt::from(-213),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                location: Location::test(3, 18),
                value: BigInt::from(-200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                location: Location::test(3, 17),
                value: BigInt::from(200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                location: Location::test(3, 17),
                value: BigInt::from(420),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowDivision {
                location: Location::test(3, 18),
                value: BigInt::from(128),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                location: Location::test(3, 17),
                value: BigInt::from(200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                location: Location::test(3, 19),
                value: BigInt::from(-100),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                location: Location::test(3, 19),
                value: BigInt::from(128),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                location: Location::test(3, 18),
                value: BigInt::from(-200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldDivision {
                location: Location::test(3, 17),
            },
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldRemainder {
                location: Location::test(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitor() {
    let input = r#"
fn main() {
    let value = -42 | -1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::test(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitor() {
    let input = r#"
fn main() {
    let value = 42 as field | 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::test(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitxor() {
    let input = r#"
fn main() {
    let value = -42 ^ -1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::test(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitxor() {
    let input = r#"
fn main() {
    let value = 42 as field ^ 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::test(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitand() {
    let input = r#"
fn main() {
    let value = -42 & -1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::test(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitand() {
    let input = r#"
fn main() {
    let value = 42 as field & 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::test(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_shift_left() {
    let input = r#"
fn main() {
    let value = -42 << 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::test(3, 18),
            },
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::test(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_shift_right() {
    let input = r#"
fn main() {
    let value = -42 >> 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::test(3, 18),
            },
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::test(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_not() {
    let input = r#"
fn main() {
    let value = ~-42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::test(3, 19),
            },
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::test(3, 19),
            },
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldNegation {
                location: Location::test(3, 19),
            },
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
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroDivision {
            location: Location::test(3, 22),
        })),
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ZeroRemainder {
                location: Location::test(3, 22),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_integer_too_large_ordinar_constant() {
    let input = r#"
fn main() {
    let invalid = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                location: Location::test(3, 19),
                inner: InferenceError::Overflow {
                    value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(zinc_const::panic::TEST_DATA_VALID),
                    is_signed: false,
                    bitlength: zinc_const::bitlength::FIELD,
                }
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                location: Location::test(3, 17),
                inner: InferenceError::Overflow {
                    value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(zinc_const::panic::TEST_DATA_VALID),
                    is_signed: false,
                    bitlength: zinc_const::bitlength::FIELD,
                }
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                location: Location::test(5, 9),
                inner: InferenceError::Overflow {
                    value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(zinc_const::panic::TEST_DATA_VALID),
                    is_signed: false,
                    bitlength: zinc_const::bitlength::FIELD,
                }
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
