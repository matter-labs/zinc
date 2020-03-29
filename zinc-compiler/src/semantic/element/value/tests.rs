//!
//! The value element tests.
//!

#![cfg(test)]

use std::convert::TryFrom;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::Value;
use crate::semantic::Error as SemanticError;

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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorOrFirstOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorOrSecondOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorXorFirstOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorXorSecondOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAndFirstOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAndSecondOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 23),
        ElementError::Value(
            ValueError::OperatorEqualsFirstOperandExpectedPrimitiveType {
                found: Type::array(Type::integer_unsigned(crate::BITLENGTH_BYTE), 3).to_string(),
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
        Location::new(5, 22),
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedUnit {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 23),
        ElementError::Value(
            ValueError::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                found: Type::array(Type::integer_unsigned(crate::BITLENGTH_BYTE), 3).to_string(),
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
        Location::new(5, 22),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedUnit {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorGreaterEqualsFirstOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorGreaterEqualsSecondOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorLesserEqualsFirstOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorLesserEqualsSecondOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorGreaterFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorGreaterSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorLesserFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorLesserSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_or_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean | integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorBitwiseOrFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_or_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer | boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorBitwiseOrSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_xor_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean ^ integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorBitwiseXorFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_xor_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer ^ boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorBitwiseXorSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_and_1st_operand_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean & integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorBitwiseAndFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_bitwise_and_2nd_operand_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = integer & boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorBitwiseAndSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorBitwiseShiftRightFirstOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorBitwiseShiftRightSecondOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAdditionFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAdditionSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorSubtractionFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorSubtractionSecondOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorMultiplicationFirstOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorMultiplicationSecondOperandExpectedInteger {
                found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorDivisionFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorDivisionSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorRemainderFirstOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorRemainderSecondOperandExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(4, 17),
        ElementError::Value(ValueError::OperatorNotExpectedBoolean {
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
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
        Location::new(4, 17),
        ElementError::Value(ValueError::OperatorBitwiseNotExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(4, 17),
        ElementError::Value(ValueError::OperatorNegationExpectedInteger {
            found: Type::boolean().to_string(),
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
        Location::new(3, 36),
        ElementError::Value(ValueError::OperatorIndexFirstOperandExpectedArray {
            found: Value::try_from(&Type::tuple(vec![Type::boolean(); 3]))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
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
        Location::new(3, 26),
        ElementError::Value(
            ValueError::OperatorIndexSecondOperandExpectedIntegerOrRange {
                found: Constant::Boolean(true).to_string(),
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
        Location::new(3, 36),
        ElementError::Value(ValueError::OperatorFieldFirstOperandExpectedTuple {
            found: Value::try_from(&Type::array(Type::boolean(), 3))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
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
        Location::new(3, 36),
        ElementError::Value(ValueError::OperatorFieldFirstOperandExpectedStructure {
            found: Value::try_from(&Type::array(Type::boolean(), 3))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
