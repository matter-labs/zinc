//!
//! The element tests.
//!

#![cfg(test)]

use std::convert::TryFrom;

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_operator_assignment_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 = 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value = X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_or_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 |= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_or_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value |= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_xor_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 ^= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_xor_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value ^= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_and_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 &= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_and_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value &= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_shift_left_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 <<= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_shift_left_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value <<= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_shift_right_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 >>= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitwise_shift_right_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value >>= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_addition_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 += 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentAdditionFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_addition_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value += X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_subtraction_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 -= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentSubtractionFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_subtraction_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value -= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_multiplication_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 *= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_multiplication_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value *= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_division_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 /= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentDivisionFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_division_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value /= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_remainder_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 %= 5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 7),
        ElementError::OperatorAssignmentRemainderFirstOperandExpectedPlace {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_remainder_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value %= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 11),
        ElementError::OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_range_1st_operand_expected_constant() {
    let input = r#"
fn main() {
    let a = 0;
    a .. 42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 7),
        ElementError::OperatorRangeFirstOperandExpectedConstant {
            found: Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::panic::TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_range_2nd_operand_expected_constant() {
    let input = r#"
fn main() {
    let b = 42;
    0 .. b
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 7),
        ElementError::OperatorRangeSecondOperandExpectedConstant {
            found: Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::panic::TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_range_inclusive_1st_operand_expected_constant() {
    let input = r#"
fn main() {
    let a = 0;
    a ..= 42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 7),
        ElementError::OperatorRangeInclusiveFirstOperandExpectedConstant {
            found: Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::panic::TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_range_inclusive_2nd_operand_expected_constant() {
    let input = r#"
fn main() {
    let b = 42;
    0 ..= b
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 7),
        ElementError::OperatorRangeInclusiveSecondOperandExpectedConstant {
            found: Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::panic::TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_or_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X || true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorOrFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_or_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = true || X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::OperatorOrSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_xor_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X ^^ true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorXorFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_xor_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = true ^^ X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::OperatorXorSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_and_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X && true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorAndFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_and_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = true && X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::OperatorAndSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X == true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorEqualsFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = true == X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::OperatorEqualsSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X != true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorNotEqualsFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = true != X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::OperatorNotEqualsSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_equals_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X >= 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorGreaterEqualsFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_equals_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 >= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_equals_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X <= 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorLesserEqualsFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_equals_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 <= X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorLesserEqualsSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X > 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorGreaterFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 > X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorGreaterSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X < 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorLesserFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 < X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorLesserSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_or_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X | 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorBitwiseOrFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_or_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 | X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorBitwiseOrSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_xor_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X ^ 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorBitwiseXorFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_xor_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 ^ X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorBitwiseXorSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_and_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X & 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorBitwiseAndFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_and_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 & X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorBitwiseAndSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_left_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X << 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_left_2nd_operand_expected_constant() {
    let input = r#"
fn main() {
    let offset = 2;
    let value = 42 << offset;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 20),
        ElementError::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
            found: Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::panic::TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_right_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X >> 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_right_2nd_operand_expected_constant() {
    let input = r#"
fn main() {
    let offset = 2;
    let value = 42 >> offset;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 20),
        ElementError::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
            found: Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::panic::TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_addition_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X + 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorAdditionFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_addition_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 + X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorAdditionSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_subtraction_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X - 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorSubtractionFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_subtraction_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 - X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorSubtractionSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_multiplication_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X * 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorMultiplicationFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_multiplication_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 * X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorMultiplicationSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_division_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X / 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorDivisionFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_division_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 / X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorDivisionSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_remainder_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X % 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorRemainderFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_remainder_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 % X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 20),
        ElementError::OperatorRemainderSecondOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_casting_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 19),
        ElementError::OperatorCastingFirstOperandExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = !X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 17),
        ElementError::OperatorNotExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_not_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = ~X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 17),
        ElementError::OperatorBitwiseNotExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_negation_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = -X;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 17),
        ElementError::OperatorNegationExpectedEvaluable {
            found: Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_index_1st_operand_expected_place_or_evaluable() {
    let input = r#"
type X = field;

fn main() {
    X[42];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 6),
        ElementError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
            found: Type::field().to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_index_2nd_operand_expected_evaluable() {
    let input = r#"
type X = field;

fn main() {
    let array = [1, 2, 3, 4, 5];
    let result = array[X];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(6, 23),
        ElementError::OperatorIndexSecondOperandExpectedEvaluable {
            found: Element::Type(Type::field()).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_field_1st_operand_expected_place_or_evaluable() {
    let input = r#"
type X = field;

fn main() {
    X.data;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 6),
        ElementError::OperatorFieldFirstOperandExpectedPlaceOrEvaluable {
            found: Type::field().to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_path_1st_operand_expected_path() {
    let input = r#"
fn main() {
    let value = 5::UNDEFINED;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 18),
        ElementError::OperatorPathFirstOperandExpectedPath {
            found: IntegerConstant::new(BigInt::from(5), false, crate::BITLENGTH_BYTE).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_path_2nd_operand_expected_identifier() {
    let input = r#"
enum Value {
    FIRST = 1,
}

fn main() {
    let value = Value::5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(7, 22),
        ElementError::OperatorPathSecondOperandExpectedIdentifier {
            found: IntegerConstant::new(BigInt::from(5), false, crate::BITLENGTH_BYTE).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
