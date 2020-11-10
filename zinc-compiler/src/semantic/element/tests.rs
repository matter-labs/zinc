//!
//! The element tests.
//!

use num::BigInt;

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentSecondOperandExpectedEvaluable {
            location: Location::test(6, 13),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitor_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 |= 5;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitor_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value |= X;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitxor_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 ^= 5;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitxor_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value ^= X;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitand_1st_operand_expected_place() {
    let input = r#"
fn main() {
    5 &= 5;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_assignment_bitand_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let mut value = 0;
    value &= X;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
            location: Location::test(6, 15),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
            location: Location::test(6, 15),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentAdditionFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentSubtractionFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentDivisionFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentRemainderFirstOperandExpectedPlace {
            location: Location::test(3, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 5),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
            location: Location::test(6, 14),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeFirstOperandExpectedConstant {
            location: Location::test(4, 5),
            found: Element::Value(
                Value::try_from_type(
                    &Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    false,
                    None,
                )
                .expect(zinc_const::panic::TEST_DATA_VALID),
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeSecondOperandExpectedConstant {
            location: Location::test(4, 10),
            found: Element::Value(
                Value::try_from_type(
                    &Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    false,
                    None,
                )
                .expect(zinc_const::panic::TEST_DATA_VALID),
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeInclusiveFirstOperandExpectedConstant {
            location: Location::test(4, 5),
            found: Element::Value(
                Value::try_from_type(
                    &Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    false,
                    None,
                )
                .expect(zinc_const::panic::TEST_DATA_VALID),
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeInclusiveSecondOperandExpectedConstant {
            location: Location::test(4, 11),
            found: Element::Value(
                Value::try_from_type(
                    &Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    false,
                    None,
                )
                .expect(zinc_const::panic::TEST_DATA_VALID),
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorOrFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorOrSecondOperandExpectedEvaluable {
            location: Location::test(5, 25),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorXorFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorXorSecondOperandExpectedEvaluable {
            location: Location::test(5, 25),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAndFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAndSecondOperandExpectedEvaluable {
            location: Location::test(5, 25),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorEqualsFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorEqualsSecondOperandExpectedEvaluable {
            location: Location::test(5, 25),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotEqualsFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotEqualsSecondOperandExpectedEvaluable {
            location: Location::test(5, 25),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterEqualsFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterEqualsSecondOperandExpectedEvaluable {
            location: Location::test(5, 23),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserEqualsFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserEqualsSecondOperandExpectedEvaluable {
            location: Location::test(5, 23),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitor_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X | 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseOrFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitor_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 | X;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseOrSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitxor_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X ^ 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseXorFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitxor_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 ^ X;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseXorSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitand_1st_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = X & 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseAndFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitand_2nd_operand_expected_evaluable() {
    let input = r#"
type X = u8;

fn main() {
    let value = 42 & X;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseAndSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
            location: Location::test(4, 23),
            found: Element::Value(
                Value::try_from_type(
                    &Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    false,
                    None,
                )
                .expect(zinc_const::panic::TEST_DATA_VALID),
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftRightSecondOperandExpectedConstant {
            location: Location::test(4, 23),
            found: Element::Value(
                Value::try_from_type(
                    &Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    false,
                    None,
                )
                .expect(zinc_const::panic::TEST_DATA_VALID),
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAdditionFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAdditionSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorSubtractionFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorSubtractionSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorMultiplicationFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorMultiplicationSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDivisionFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDivisionSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRemainderFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRemainderSecondOperandExpectedEvaluable {
            location: Location::test(5, 22),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorCastingFirstOperandExpectedEvaluable {
            location: Location::test(5, 17),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotExpectedEvaluable {
            location: Location::test(5, 18),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseNotExpectedEvaluable {
            location: Location::test(5, 18),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNegationExpectedEvaluable {
            location: Location::test(5, 18),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
            location: Location::test(5, 5),
            found: Element::Type(Type::field(None)).to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorIndexSecondOperandExpectedEvaluable {
            location: Location::test(6, 24),
            found: Element::Type(Type::field(None)).to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDotFirstOperandExpectedPlaceOrEvaluable {
            location: Location::test(5, 5),
            found: Element::Type(Type::field(None)).to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorPathFirstOperandExpectedPath {
            location: Location::test(3, 17),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(3, 17),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorPathSecondOperandExpectedIdentifier {
            location: Location::test(7, 24),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::test(7, 24),
                BigInt::from(5),
                false,
                zinc_const::bitlength::BYTE,
                true,
            )))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
