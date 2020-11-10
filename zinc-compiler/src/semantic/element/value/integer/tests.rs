//!
//! The integer value element tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::integer::Integer as IntegerValue;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_types_mismatch_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 == integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorEqualsTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_not_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 != integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNotEqualsTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 >= integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterEqualsTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_equals() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 <= integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserEqualsTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 > integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorGreaterTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 < integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorLesserTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitor() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 | integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseOrTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitxor() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 ^ integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseXorTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitand() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 & integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseAndTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_addition() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 + integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorAdditionTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_subtraction() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 - integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorSubtractionTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_multiplication() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 * integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorMultiplicationTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_division() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 / integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDivisionTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_remainder() {
    let input = r#"
fn main() {
    let integer_64: u64 = 42;
    let integer_128: u128 = 64;
    let value = integer_64 % integer_128;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRemainderTypesMismatch {
            location: Location::test(5, 17),
            first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
            second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
            location: Location::test(4, 28),
            found: IntegerValue::new(
                Some(Location::test(4, 28)),
                true,
                zinc_const::bitlength::BYTE,
                true,
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
            location: Location::test(4, 28),
            found: IntegerValue::new(
                Some(Location::test(4, 28)),
                true,
                zinc_const::bitlength::BYTE,
                true,
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDivisionFieldOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorRemainderFieldOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(5, 5),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(5, 5),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(5, 5),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(4, 5),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(4, 5),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitor() {
    let input = r#"
fn main() {
    let signed_1: i8 = 42;
    let signed_2: i8 = 1;
    let value = signed_1 | signed_2;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseSignedOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitor() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 | field_2;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitxor() {
    let input = r#"
fn main() {
    let signed_1: i8 = 42;
    let signed_2: i8 = 1;
    let value = signed_1 ^ signed_2;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseSignedOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitxor() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 ^ field_2;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitand() {
    let input = r#"
fn main() {
    let signed_1: i8 = 42;
    let signed_2: i8 = 1;
    let value = signed_1 & signed_2;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseSignedOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitand() {
    let input = r#"
fn main() {
    let field_1: field = 42;
    let field_2: field = 1;
    let value = field_1 & field_2;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(5, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_shift_left() {
    let input = r#"
fn main() {
    let signed_1: i8 = 42;
    let value = signed_1 << 1;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseSignedOperandForbidden {
            location: Location::test(4, 17),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(4, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_shift_right() {
    let input = r#"
fn main() {
    let signed_1: i8 = 42;
    let value = signed_1 >> 1;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseSignedOperandForbidden {
            location: Location::test(4, 17),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(4, 17),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_not() {
    let input = r#"
fn main() {
    let signed: i8 = 42;
    let value = ~signed;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseSignedOperandForbidden {
            location: Location::test(4, 18),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorBitwiseFieldOperandForbidden {
            location: Location::test(4, 18),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorNegationFieldOperandForbidden {
            location: Location::test(4, 18),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
