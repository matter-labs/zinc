//!
//! The constant element tests.
//!

use num::BigInt;

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::constant::array::Array as ArrayConstant;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::string::String as StringConstant;
use crate::semantic::element::constant::tuple::Tuple as TupleConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_operator_range_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true .. 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_range_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 .. true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeSecondOperandExpectedInteger {
            location: Location::test(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 23), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_range_inclusive_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true ..= 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeInclusiveFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_range_inclusive_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 ..= true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRangeInclusiveSecondOperandExpectedInteger {
            location: Location::test(3, 24),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 24), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_or_1st_expected_boolean() {
    let input = r#"
fn main() {
    let value = 42 || true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorOrFirstOperandExpectedBoolean {
            location: Location::test(3, 17),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 17),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_or_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true || 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorOrSecondOperandExpectedBoolean {
            location: Location::test(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_xor_1st_expected_boolean() {
    let input = r#"
fn main() {
    let value = 42 ^^ true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorXorFirstOperandExpectedBoolean {
            location: Location::test(3, 17),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 17),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_xor_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true ^^ 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorXorSecondOperandExpectedBoolean {
            location: Location::test(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_and_1st_expected_boolean() {
    let input = r#"
fn main() {
    let value = 42 && true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAndFirstOperandExpectedBoolean {
            location: Location::test(3, 17),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 17),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_and_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true && 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAndSecondOperandExpectedBoolean {
            location: Location::test(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_1st_expected_primitive() {
    let input = r#"
fn main() {
    let value = "string" == 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorEqualsFirstOperandExpectedPrimitiveType {
            location: Location::test(3, 17),
            found: Constant::String(StringConstant::new(
                Location::test(3, 17),
                "string".to_owned(),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_2nd_expected_unit() {
    let input = r#"
fn main() {
    let value = () == 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorEqualsSecondOperandExpectedUnit {
            location: Location::test(3, 23),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 23),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true == 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorEqualsSecondOperandExpectedBoolean {
            location: Location::test(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 == true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorEqualsSecondOperandExpectedInteger {
            location: Location::test(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 23), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_1st_expected_primitive() {
    let input = r#"
fn main() {
    let value = "string" != 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
            location: Location::test(3, 17),
            found: Constant::String(StringConstant::new(
                Location::test(3, 17),
                "string".to_owned(),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_2nd_expected_unit() {
    let input = r#"
fn main() {
    let value = () != 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotEqualsSecondOperandExpectedUnit {
            location: Location::test(3, 23),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 23),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true != 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotEqualsSecondOperandExpectedBoolean {
            location: Location::test(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::test(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 != true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotEqualsSecondOperandExpectedInteger {
            location: Location::test(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 23), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_equals_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true >= 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterEqualsFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 >= true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterEqualsSecondOperandExpectedInteger {
            location: Location::test(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 23), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_equals_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true <= 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserEqualsFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 <= true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserEqualsSecondOperandExpectedInteger {
            location: Location::test(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 23), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true > 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 > true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true < 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 < true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitor_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true | 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseOrFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitor_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 | true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseOrSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitxor_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true ^ 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseXorFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitxor_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 ^ true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseXorSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitand_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true & 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseAndFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitand_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 & true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseAndSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_left_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true << 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_left_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 << true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
            location: Location::test(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 23), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_right_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true >> 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_right_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 >> true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
            location: Location::test(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 23), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_addition_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true + 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAdditionFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_addition_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 + true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAdditionSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_subtraction_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true - 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorSubtractionFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_subtraction_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 - true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorSubtractionSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_multiplication_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true * 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorMultiplicationFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_multiplication_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 * true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorMultiplicationSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_division_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true / 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDivisionFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_division_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 / true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDivisionSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_remainder_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true % 42;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRemainderFirstOperandExpectedInteger {
            location: Location::test(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 17), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_remainder_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 % true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRemainderSecondOperandExpectedInteger {
            location: Location::test(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 22), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_casting_to_invalid_type_const() {
    let input = r#"
fn main() {
    const VALUE: u8 = 42;
    const RESULT: bool = VALUE;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorCastingTypesMismatch {
            location: Location::test(4, 26),
            inner: CastingError::CastingToInvalidType {
                from: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
                to: Type::boolean(None).to_string(),
            },
            reference: Location::test(4, 19),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_expected_boolean() {
    let input = r#"
fn main() {
    let value = !42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::OperatorNotExpectedBoolean {
        location: Location::test(3, 18),
        found: Constant::Integer(IntegerConstant::new(
            Location::test(3, 18),
            BigInt::from(42),
            false,
            zinc_const::bitlength::BYTE,
            true,
        ))
        .to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_not_expected_integer() {
    let input = r#"
fn main() {
    let value = ~true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseNotExpectedInteger {
            location: Location::test(3, 18),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 18), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_negation_expected_integer() {
    let input = r#"
fn main() {
    let value = -true;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNegationExpectedInteger {
            location: Location::test(3, 18),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 18), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_index_1st_operand_expected_array() {
    let input = r#"
fn main() {
    const VALUE: bool = (true, false, true)[1];
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorIndexFirstOperandExpectedArray {
            location: Location::test(3, 25),
            found: Constant::Tuple(TupleConstant::new_with_values(
                Location::test(3, 25),
                vec![
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 26), true)),
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 32), false)),
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 39), true)),
                ],
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_index_2nd_operand_expected_integer_or_range() {
    let input = r#"
fn main() {
    const VALUE: u8 = [1, 2, 3][true];
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorIndexSecondOperandExpectedIntegerOrRange {
            location: Location::test(3, 33),
            found: Constant::Boolean(BooleanConstant::new(Location::test(3, 33), true)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_field_1st_operand_expected_tuple() {
    let input = r#"
fn main() {
    const VALUE: bool = [true, true, false].1;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDotFirstOperandExpectedTuple {
            location: Location::test(3, 25),
            found: Constant::Array(ArrayConstant::new_with_values(
                Location::test(3, 25),
                Type::boolean(None),
                vec![
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 26), true)),
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 32), true)),
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 38), false)),
                ],
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_field_1st_operand_expected_structure() {
    let input = r#"
fn main() {
    const VALUE: bool = [true, true, false].first;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDotFirstOperandExpectedInstance {
            location: Location::test(3, 25),
            found: Constant::Array(ArrayConstant::new_with_values(
                Location::test(3, 25),
                Type::boolean(None),
                vec![
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 26), true)),
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 32), true)),
                    Constant::Boolean(BooleanConstant::new(Location::test(3, 38), false)),
                ],
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
