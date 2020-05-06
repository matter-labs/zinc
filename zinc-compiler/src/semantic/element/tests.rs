//!
//! The element tests.
//!

#![cfg(test)]

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
        ElementError::OperatorAssignmentFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentSecondOperandExpectedEvaluable {
            location: Location::new(6, 13),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
            location: Location::new(6, 15),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
            location: Location::new(6, 15),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentAdditionFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentSubtractionFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentDivisionFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAssignmentRemainderFirstOperandExpectedPlace {
            location: Location::new(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(3, 5),
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
        ElementError::OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
            location: Location::new(6, 14),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorRangeFirstOperandExpectedConstant {
            location: Location::new(4, 5),
            found: Value::try_from_type(&Type::integer_unsigned(None, crate::BITLENGTH_BYTE), None)
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
        ElementError::OperatorRangeSecondOperandExpectedConstant {
            location: Location::new(4, 10),
            found: Value::try_from_type(&Type::integer_unsigned(None, crate::BITLENGTH_BYTE), None)
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
        ElementError::OperatorRangeInclusiveFirstOperandExpectedConstant {
            location: Location::new(4, 5),
            found: Value::try_from_type(&Type::integer_unsigned(None, crate::BITLENGTH_BYTE), None)
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
        ElementError::OperatorRangeInclusiveSecondOperandExpectedConstant {
            location: Location::new(4, 11),
            found: Value::try_from_type(&Type::integer_unsigned(None, crate::BITLENGTH_BYTE), None)
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
        ElementError::OperatorOrFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorOrSecondOperandExpectedEvaluable {
            location: Location::new(5, 25),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorXorFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorXorSecondOperandExpectedEvaluable {
            location: Location::new(5, 25),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAndFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAndSecondOperandExpectedEvaluable {
            location: Location::new(5, 25),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorEqualsFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorEqualsSecondOperandExpectedEvaluable {
            location: Location::new(5, 25),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorNotEqualsFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorNotEqualsSecondOperandExpectedEvaluable {
            location: Location::new(5, 25),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorGreaterEqualsFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
            location: Location::new(5, 23),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorLesserEqualsFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorLesserEqualsSecondOperandExpectedEvaluable {
            location: Location::new(5, 23),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorGreaterFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorGreaterSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorLesserFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorLesserSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseOrFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseOrSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseXorFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseXorSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseAndFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseAndSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
            location: Location::new(4, 23),
            found: Value::try_from_type(&Type::integer_unsigned(None, crate::BITLENGTH_BYTE), None)
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
        ElementError::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
            location: Location::new(4, 23),
            found: Value::try_from_type(&Type::integer_unsigned(None, crate::BITLENGTH_BYTE), None)
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
        ElementError::OperatorAdditionFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorAdditionSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorSubtractionFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorSubtractionSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorMultiplicationFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorMultiplicationSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorDivisionFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorDivisionSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorRemainderFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorRemainderSecondOperandExpectedEvaluable {
            location: Location::new(5, 22),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorCastingFirstOperandExpectedEvaluable {
            location: Location::new(5, 17),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorNotExpectedEvaluable {
            location: Location::new(5, 18),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorBitwiseNotExpectedEvaluable {
            location: Location::new(5, 18),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorNegationExpectedEvaluable {
            location: Location::new(5, 18),
            found: Element::Type(Type::integer_unsigned(None, crate::BITLENGTH_BYTE)).to_string(),
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
        ElementError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
            location: Location::new(5, 5),
            found: Type::field(None).to_string(),
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
        ElementError::OperatorIndexSecondOperandExpectedEvaluable {
            location: Location::new(6, 24),
            found: Element::Type(Type::field(None)).to_string(),
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
        ElementError::OperatorFieldFirstOperandExpectedPlaceOrEvaluable {
            location: Location::new(5, 5),
            found: Type::field(None).to_string(),
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
        ElementError::OperatorPathFirstOperandExpectedPath {
            location: Location::new(3, 17),
            found: IntegerConstant::new(
                Location::new(3, 17),
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )
            .to_string(),
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
        ElementError::OperatorPathSecondOperandExpectedIdentifier {
            location: Location::new(7, 24),
            found: IntegerConstant::new(
                Location::new(7, 24),
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
