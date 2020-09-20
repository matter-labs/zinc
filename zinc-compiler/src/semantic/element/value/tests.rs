//!
//! The value element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::Value;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_operator_or_1st_operand_expected_boolean() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer || boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorOrFirstOperandExpectedBoolean {
            location: Location::test(5, 17),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_or_2nd_operand_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean || integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorOrSecondOperandExpectedBoolean {
            location: Location::test(5, 28),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_xor_1st_operand_expected_boolean() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer ^^ boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorXorFirstOperandExpectedBoolean {
            location: Location::test(5, 17),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_xor_2nd_operand_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean ^^ integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorXorSecondOperandExpectedBoolean {
            location: Location::test(5, 28),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_and_1st_operand_expected_boolean() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer && boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorAndFirstOperandExpectedBoolean {
            location: Location::test(5, 17),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_and_2nd_operand_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean && integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorAndSecondOperandExpectedBoolean {
            location: Location::test(5, 28),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_1st_operand_expected_primitive() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let integer = 42;
    let value = array == integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorEqualsFirstOperandExpectedPrimitiveType {
                location: Location::test(5, 17),
                found: Type::array(
                    Some(Location::test(5, 17)),
                    Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    3,
                )
                .to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_2nd_operand_expected_unit() {
    let input = r#"
fn main() {
    let unit = ();
    let integer = 42;
    let value = unit == integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedUnit {
            location: Location::test(5, 25),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_2nd_operand_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean == integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedBoolean {
            location: Location::test(5, 28),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_equals_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = false;
    let value = integer == boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedInteger {
            location: Location::test(5, 28),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_1st_operand_expected_primitive() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let integer = 42;
    let value = array != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                location: Location::test(5, 17),
                found: Type::array(
                    Some(Location::test(5, 17)),
                    Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                    3,
                )
                .to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_2nd_operand_expected_unit() {
    let input = r#"
fn main() {
    let unit = ();
    let integer = 42;
    let value = unit != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedUnit {
            location: Location::test(5, 25),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_2nd_operand_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedBoolean {
            location: Location::test(5, 28),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_equals_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = false;
    let value = integer != boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedInteger {
            location: Location::test(5, 28),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_equals_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean >= integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorGreaterEqualsFirstOperandExpectedInteger {
                location: Location::test(5, 17),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_equals_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer >= boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorGreaterEqualsSecondOperandExpectedInteger {
                location: Location::test(5, 28),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_equals_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean <= integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorLesserEqualsFirstOperandExpectedInteger {
                location: Location::test(5, 17),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_equals_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer <= boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorLesserEqualsSecondOperandExpectedInteger {
                location: Location::test(5, 28),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean > integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorGreaterFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_greater_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer > boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorGreaterSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean < integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorLesserFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_lesser_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer < boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorLesserSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitor_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean | integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorBitwiseOrFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitor_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer | boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorBitwiseOrSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitxor_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean ^ integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorBitwiseXorFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitxor_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer ^ boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorBitwiseXorSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitand_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean & integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorBitwiseAndFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitand_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer & boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorBitwiseAndSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_left_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    const INTEGER: u8 = 42;
    let value = boolean << INTEGER;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                location: Location::test(5, 17),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_left_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    const BOOLEAN: bool = true;
    let value = integer << BOOLEAN;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                location: Location::test(5, 28),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_right_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    const INTEGER: u8 = 42;
    let value = boolean >> INTEGER;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                location: Location::test(5, 17),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_shift_right_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    const BOOLEAN: bool = true;
    let value = integer >> BOOLEAN;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                location: Location::test(5, 28),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_addition_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean + integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorAdditionFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_addition_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer + boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorAdditionSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_subtraction_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean - integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorSubtractionFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_subtraction_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer - boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorSubtractionSecondOperandExpectedInteger {
                location: Location::test(5, 27),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_multiplication_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean * integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorMultiplicationFirstOperandExpectedInteger {
                location: Location::test(5, 17),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_multiplication_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer * boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorMultiplicationSecondOperandExpectedInteger {
                location: Location::test(5, 27),
                found: Type::boolean(None).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_division_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean / integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorDivisionFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_division_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer / boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorDivisionSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_remainder_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean % integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorRemainderFirstOperandExpectedInteger {
            location: Location::test(5, 17),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_remainder_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer % boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorRemainderSecondOperandExpectedInteger {
            location: Location::test(5, 27),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_not_expected_boolean() {
    let input = r#"
fn main() {
    let integer = 42;
    let value = !integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorNotExpectedBoolean {
            location: Location::test(4, 18),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_not_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let value = ~boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorBitwiseNotExpectedInteger {
            location: Location::test(4, 18),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_negation_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let value = -boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorNegationExpectedInteger {
            location: Location::test(4, 18),
            found: Type::boolean(None).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_index_1st_operand_expected_array() {
    let input = r#"
fn main() {
    let value = (true, false, true)[1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorIndexFirstOperandExpectedArray {
            location: Location::test(3, 17),
            found: Value::try_from_type(
                &Type::tuple(Some(Location::test(3, 17)), vec![Type::boolean(None); 3]),
                false,
                None,
            )
            .expect(zinc_const::panic::TEST_DATA_VALID)
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
    let value = [1, 2, 3][true];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(
            ValueError::OperatorIndexSecondOperandExpectedIntegerOrRange {
                location: Location::test(3, 27),
                found: Constant::Boolean(BooleanConstant::new(Location::test(3, 27), true))
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
    let value = [true, true, false].1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorDotFirstOperandExpectedTuple {
            location: Location::test(3, 17),
            found: Value::try_from_type(
                &Type::array(Some(Location::test(3, 17)), Type::boolean(None), 3),
                true,
                None,
            )
            .expect(zinc_const::panic::TEST_DATA_VALID)
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
    let value = [true, true, false].first;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Value(ValueError::OperatorDotFirstOperandExpectedInstance {
            location: Location::test(3, 17),
            found: Value::try_from_type(
                &Type::array(Some(Location::test(3, 17)), Type::boolean(None), 3),
                true,
                None,
            )
            .expect(zinc_const::panic::TEST_DATA_VALID)
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
