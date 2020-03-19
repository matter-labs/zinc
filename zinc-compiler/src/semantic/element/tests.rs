//!
//! The element tests.
//!

#![cfg(test)]

use std::convert::TryFrom;

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::Error as SemanticError;

#[test]
fn error_element_assignment_1st_expected_place() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_addition_1st_expected_place() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_addition_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_subtraction_1st_expected_place() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_subtraction_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_multiplication_1st_expected_place() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_multiplication_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_division_1st_expected_place() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_division_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_remainder_1st_expected_place() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_assignment_remainder_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_range_1st_expected_constant() {
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
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_range_2nd_expected_constant() {
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
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_range_inclusive_1st_expected_constant() {
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
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_range_inclusive_2nd_expected_constant() {
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
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_or_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_or_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_xor_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_xor_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_and_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_and_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_equals_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_equals_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_not_equals_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_not_equals_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_greater_equals_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_greater_equals_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_lesser_equals_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_lesser_equals_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_greater_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_greater_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_lesser_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_lesser_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_or_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_or_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_xor_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_xor_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_and_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_and_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_shift_left_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_shift_left_2nd_expected_constant() {
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
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_shift_right_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_shift_right_2nd_expected_constant() {
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
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_addition_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_addition_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_subtraction_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_subtraction_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_multiplication_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_multiplication_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_division_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_division_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_remainder_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_remainder_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_casting_1st_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_not_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_bitwise_not_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_negation_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_index_1st_expected_place_or_evaluable() {
    let input = r#"
fn main() {
    5[42];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 6),
        ElementError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_index_2nd_expected_evaluable() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_field_1st_expected_place_or_evaluable() {
    let input = r#"
fn main() {
    5.data;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 6),
        ElementError::OperatorFieldFirstOperandExpectedPlaceOrEvaluable {
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_path_1st_expected_path() {
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

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_path_2nd_expected_member_string() {
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
        ElementError::OperatorPathSecondOperandExpectedMemberString {
            found: IntegerConstant::new(BigInt::from(5), false, crate::BITLENGTH_BYTE).to_string(),
        },
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
