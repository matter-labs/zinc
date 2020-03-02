//!
//! The integer constant element tests.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::lexical::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn error_element_constant_integer_inference_constant() {
    let input = r#"
fn main() {
    let invalid = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 19),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                    .to_owned(),
                crate::BITLENGTH_FIELD,
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_inference_constant_loop_bounds() {
    let input = r#"
fn main() {
    for i in 0..0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff {}
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                    .to_owned(),
                crate::BITLENGTH_FIELD,
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_inference_constant_pattern_match() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff => 10,
        2 => 20,
        _ => 30,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 9),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                    .to_owned(),
                crate::BITLENGTH_FIELD,
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater_equals() {
    let input = r#"
fn main() {
    let value = 42 as u64 >= 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser_equals() {
    let input = r#"
fn main() {
    let value = 42 as u64 <= 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_greater() {
    let input = r#"
fn main() {
    let value = 42 as u64 > 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_lesser() {
    let input = r#"
fn main() {
    let value = 42 as u64 < 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_addition() {
    let input = r#"
fn main() {
    let value = 42 as u64 + 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_subtraction() {
    let input = r#"
fn main() {
    let value = 42 as u64 - 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_multiplication() {
    let input = r#"
fn main() {
    let value = 42 as u64 * 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_division() {
    let input = r#"
fn main() {
    let value = 42 as u64 / 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_types_mismatch_remainder() {
    let input = r#"
fn main() {
    let value = 42 as u64 % 69 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder(
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 8).to_string(),
                Type::integer_unsigned(crate::BITLENGTH_BYTE * 16).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_addition_signed_negative() {
    let input = r#"
fn main() {
    let value = -120 + (-50);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition(
                BigInt::from(-170),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_addition_signed_positive() {
    let input = r#"
fn main() {
    let value = 42 as i8 + 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition(
                BigInt::from(142),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_addition_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 + 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition(
                BigInt::from(297),
                Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_subtraction_signed_negative() {
    let input = r#"
fn main() {
    let value = -42 - 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction(
                BigInt::from(-142),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_subtraction_signed_positive() {
    let input = r#"
fn main() {
    let value = (50 as i8) - (-100);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 28),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction(
                BigInt::from(150),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_subtraction_unsigned_negative() {
    let input = r#"
fn main() {
    let value = 42 - 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction(
                BigInt::from(-213),
                Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_multiplication_signed_negative() {
    let input = r#"
fn main() {
    let value = -100 * (2 as i8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication(
                BigInt::from(-200),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_multiplication_signed_positive() {
    let input = r#"
fn main() {
    let value = 100 as i8 * 2 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 27),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication(
                BigInt::from(200),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_multiplication_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 * 10;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication(
                BigInt::from(420),
                Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_division_signed_positive() {
    let input = r#"
fn main() {
    let value = -128 / (-1);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowDivision(
                BigInt::from(128),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_casting_signed_positive() {
    let input = r#"
fn main() {
    let value = 200 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 21),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting(
                BigInt::from(200),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_casting_unsigned_negative() {
    let input = r#"
fn main() {
    let value = (-100 as u8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 23),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting(
                BigInt::from(-100),
                Type::integer(false, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_negation_signed_positive() {
    let input = r#"
fn main() {
    let value = --128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation(
                BigInt::from(128),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_overflow_negation_unsigned_negative() {
    let input = r#"
fn main() {
    let value = -200;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation(
                BigInt::from(-200),
                Type::integer(true, crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_zero_division() {
    let input = r#"
fn main() {
    let value = 42 / 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroDivision)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_integer_zero_remainder() {
    let input = r#"
fn main() {
    let value = 42 % 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroRemainder)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
