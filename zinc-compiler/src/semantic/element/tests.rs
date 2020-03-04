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
        ElementError::OperatorAssignmentFirstOperandExpectedPlace(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorAssignmentSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAssignmentAdditionFirstOperandExpectedPlace(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorAssignmentAdditionSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAssignmentSubtractionFirstOperandExpectedPlace(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAssignmentMultiplicationFirstOperandExpectedPlace(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAssignmentDivisionFirstOperandExpectedPlace(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorAssignmentDivisionSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAssignmentRemainderFirstOperandExpectedPlace(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorAssignmentRemainderSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorRangeFirstOperandExpectedConstant(
            Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        ),
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
        ElementError::OperatorRangeSecondOperandExpectedConstant(
            Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        ),
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
        ElementError::OperatorRangeInclusiveFirstOperandExpectedConstant(
            Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        ),
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
        ElementError::OperatorRangeInclusiveSecondOperandExpectedConstant(
            Value::try_from(&Type::integer_unsigned(crate::BITLENGTH_BYTE))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        ),
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
        ElementError::OperatorOrFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorOrSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorXorFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorXorSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAndFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAndSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorEqualsFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorEqualsSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorNotEqualsFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorNotEqualsSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorGreaterEqualsFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorGreaterEqualsSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorLesserEqualsFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorLesserEqualsSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorGreaterFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorGreaterSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorLesserFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorLesserSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAdditionFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorAdditionSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorSubtractionFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorSubtractionSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorMultiplicationFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorMultiplicationSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorDivisionFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorDivisionSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorRemainderFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorRemainderSecondOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorCastingFirstOperandExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorNotExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorNegationExpectedEvaluable(
            Element::Type(Type::integer_unsigned(crate::BITLENGTH_BYTE)).to_string(),
        ),
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
        ElementError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorIndexSecondOperandExpectedEvaluable(
            Element::Type(Type::field()).to_string(),
        ),
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
        ElementError::OperatorFieldFirstOperandExpectedPlaceOrEvaluable(
            Element::Constant(Constant::Integer(IntegerConstant::new(
                BigInt::from(5),
                false,
                crate::BITLENGTH_BYTE,
            )))
            .to_string(),
        ),
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
        ElementError::OperatorPathFirstOperandExpectedPath(
            IntegerConstant::new(BigInt::from(5), false, crate::BITLENGTH_BYTE).to_string(),
        ),
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
        ElementError::OperatorPathSecondOperandExpectedMemberString(
            IntegerConstant::new(BigInt::from(5), false, crate::BITLENGTH_BYTE).to_string(),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
