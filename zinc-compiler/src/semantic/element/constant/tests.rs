//!
//! The constant element tests.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::constant::array::Array as ArrayConstant;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::string::String as StringConstant;
use crate::semantic::element::constant::tuple::Tuple as TupleConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_operator_range_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true .. 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorRangeFirstOperandExpectedInteger {
            location: Location::new(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorRangeSecondOperandExpectedInteger {
            location: Location::new(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 23), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorRangeInclusiveFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorRangeInclusiveSecondOperandExpectedInteger {
                location: Location::new(3, 24),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 24), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorOrFirstOperandExpectedBoolean {
            location: Location::new(3, 17),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 17),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorOrSecondOperandExpectedBoolean {
            location: Location::new(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorXorFirstOperandExpectedBoolean {
            location: Location::new(3, 17),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 17),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorXorSecondOperandExpectedBoolean {
            location: Location::new(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorAndFirstOperandExpectedBoolean {
            location: Location::new(3, 17),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 17),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorAndSecondOperandExpectedBoolean {
            location: Location::new(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorEqualsFirstOperandExpectedPrimitiveType {
                location: Location::new(3, 17),
                found: Constant::String(StringConstant::new(
                    Location::new(3, 17),
                    "string".to_owned(),
                ))
                .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedUnit {
            location: Location::new(3, 23),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 23),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedBoolean {
            location: Location::new(3, 25),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 25),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedInteger {
            location: Location::new(3, 23),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 23), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                location: Location::new(3, 17),
                found: Constant::String(StringConstant::new(
                    Location::new(3, 17),
                    "string".to_owned(),
                ))
                .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedUnit {
            location: Location::new(3, 23),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 23),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorNotEqualsSecondOperandExpectedBoolean {
                location: Location::new(3, 25),
                found: Constant::Integer(IntegerConstant::new(
                    Location::new(3, 25),
                    BigInt::from(42),
                    false,
                    zinc_const::bitlength::BYTE,
                    true,
                ))
                .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorNotEqualsSecondOperandExpectedInteger {
                location: Location::new(3, 23),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 23), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorGreaterEqualsFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorGreaterEqualsSecondOperandExpectedInteger {
                location: Location::new(3, 23),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 23), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorLesserEqualsFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorLesserEqualsSecondOperandExpectedInteger {
                location: Location::new(3, 23),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 23), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorGreaterFirstOperandExpectedInteger {
            location: Location::new(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorGreaterSecondOperandExpectedInteger {
            location: Location::new(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorLesserFirstOperandExpectedInteger {
            location: Location::new(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorLesserSecondOperandExpectedInteger {
            location: Location::new(3, 22),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseOrFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseOrSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseXorFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseXorSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseAndFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseAndSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                location: Location::new(3, 23),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 23), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                location: Location::new(3, 23),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 23), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorAdditionFirstOperandExpectedInteger {
            location: Location::new(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorAdditionSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorSubtractionFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorSubtractionSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorMultiplicationFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorMultiplicationSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorDivisionFirstOperandExpectedInteger {
            location: Location::new(3, 17),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorDivisionSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorRemainderFirstOperandExpectedInteger {
                location: Location::new(3, 17),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 17), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorRemainderSecondOperandExpectedInteger {
                location: Location::new(3, 22),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 22), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Casting {
            location: Location::new(4, 26),
            inner: CastingError::CastingToInvalidType {
                from: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
                to: Type::boolean(None).to_string(),
            },
            reference: Location::new(4, 19),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorNotExpectedBoolean {
            location: Location::new(3, 18),
            found: Constant::Integer(IntegerConstant::new(
                Location::new(3, 18),
                BigInt::from(42),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorBitwiseNotExpectedInteger {
            location: Location::new(3, 18),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 18), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorNegationExpectedInteger {
            location: Location::new(3, 18),
            found: Constant::Boolean(BooleanConstant::new(Location::new(3, 18), true)).to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorIndexFirstOperandExpectedArray {
            location: Location::new(3, 25),
            found: Constant::Tuple(TupleConstant::new_with_values(
                Location::new(3, 25),
                vec![
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 26), true)),
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 32), false)),
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 39), true)),
                ],
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(
            ConstantError::OperatorIndexSecondOperandExpectedIntegerOrRange {
                location: Location::new(3, 33),
                found: Constant::Boolean(BooleanConstant::new(Location::new(3, 33), true))
                    .to_string(),
            },
        ),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorDotFirstOperandExpectedTuple {
            location: Location::new(3, 25),
            found: Constant::Array(ArrayConstant::new_with_values(
                Location::new(3, 25),
                Type::boolean(None),
                vec![
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 26), true)),
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 32), true)),
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 38), false)),
                ],
            ))
            .to_string(),
        }),
    )));

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

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::OperatorDotFirstOperandExpectedInstance {
            location: Location::new(3, 25),
            found: Constant::Array(ArrayConstant::new_with_values(
                Location::new(3, 25),
                Type::boolean(None),
                vec![
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 26), true)),
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 32), true)),
                    Constant::Boolean(BooleanConstant::new(Location::new(3, 38), false)),
                ],
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
