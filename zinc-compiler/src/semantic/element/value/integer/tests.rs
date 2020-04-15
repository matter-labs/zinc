//!
//! The integer value element tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::element::value::integer::Integer as IntegerValue;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_types_mismatch_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 == integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchEquals {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_not_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 != integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchNotEquals {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 >= integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchGreaterEquals {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 <= integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchLesserEquals {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 > integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchGreater {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 < integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchLesser {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_or() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 | integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchBitwiseOr {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_xor() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 ^ integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchBitwiseXor {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitwise_and() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 & integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchBitwiseAnd {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 + integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchAddition {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 - integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchSubtraction {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 * integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchMultiplication {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 / integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchDivision {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let integer_64: u64 = 42;
    let integer_128: u128 = 69;
    let value = integer_64 % integer_128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 28),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::TypesMismatchRemainder {
                first: Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                second: Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
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
    let first = 168;
    let result = first << -2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
                found: IntegerValue::new(true, crate::BITLENGTH_BYTE).to_string(),
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
    let first = 42;
    let result = first >> -2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 24),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                found: IntegerValue::new(true, crate::BITLENGTH_BYTE).to_string(),
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
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 / field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldDivision,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_remainder() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 % field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldRemainder,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_assignment_or() {
    let input = r#"
fn main() {
    let mut field_1: field = 42;
    let field_2: field = 1;
    field_1 |= field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 13),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_assignment_xor() {
    let input = r#"
fn main() {
    let mut field_1: field = 42;
    let field_2: field = 1;
    field_1 ^= field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 13),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_assignment_and() {
    let input = r#"
fn main() {
    let mut field_1: field = 42;
    let field_2: field = 1;
    field_1 &= field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 13),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_assignment_shift_left() {
    let input = r#"
fn main() {
    let mut field_1: field = 42;
    field_1 <<= 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 13),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_assignment_shift_right() {
    let input = r#"
fn main() {
    let mut field_1: field = 42;
    field_1 >>= 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 13),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_or() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 | field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_xor() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 ^ field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_and() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 & field_2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_shift_left() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let value = field_1 << 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 25),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_shift_right() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let value = field_1 >> 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 25),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_not() {
    let input = r#"
fn main() {
    let value: field = 42;
    let value = ~value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 17),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldBitwise,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_negation() {
    let input = r#"
fn main() {
    let value: field = 42;
    let value = -value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 17),
        ElementError::Value(ValueError::Integer(
            IntegerValueError::ForbiddenFieldNegation,
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
