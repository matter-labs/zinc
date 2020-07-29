//!
//! The constant integer element tests.
//!

#![cfg(test)]

use std::str::FromStr;

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_minimal_bitlength() {
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("0").unwrap_or_default(),
            false,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("255").unwrap_or_default(),
            false,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("256").unwrap_or_default(),
            false,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("65535").unwrap_or_default(),
            false,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("65536").unwrap_or_default(),
            false,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 3),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("4294967295").unwrap_or_default(),
            false,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 4),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("4294967296").unwrap_or_default(),
            false,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 5),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("18446744073709551615").unwrap_or_default(),
            false,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 8),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("18446744073709551616").unwrap_or_default(),
            false,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 9),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("-128").unwrap_or_default(),
            true,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("127").unwrap_or_default(),
            true,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("128").unwrap_or_default(),
            true,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("32767").unwrap_or_default(),
            true,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("32768").unwrap_or_default(),
            true,
            Location::default()
        ),
        Ok(zinc_const::bitlength::BYTE * 3),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("2147483647").unwrap_or_default(),
            true,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 4),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("2147483648").unwrap_or_default(),
            true,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 5),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("9223372036854775807").unwrap_or_default(),
            true,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 8),
    );
    assert_eq!(
        IntegerConstant::minimal_bitlength(
            &BigInt::from_str("9223372036854775808").unwrap_or_default(),
            true,
            Location::default(),
        ),
        Ok(zinc_const::bitlength::BYTE * 9),
    );
}

#[test]
fn ok_literal_inference() {
    // none of the operands are literals
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(256),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                false,
            ),
        ),
        (None, None),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(65535),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                false,
            ),
        ),
        (None, None),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                false,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(256),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
        ),
        (None, None),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                false,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(65535),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
        ),
        (None, None),
    );

    // the first operand is a literal
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(256),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
        ),
        (
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
            None
        ),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(65535),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
        ),
        (
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
            None
        ),
    );

    // the second operand is a literal
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(256),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
        ),
        (
            None,
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            ))
        ),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(65535),
                false,
                zinc_const::bitlength::BYTE * 2,
                false,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
        ),
        (
            None,
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            ))
        ),
    );

    // both operands are literals
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(256),
                false,
                zinc_const::bitlength::BYTE * 2,
                true,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
        ),
        (
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
        ),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(65535),
                false,
                zinc_const::bitlength::BYTE * 2,
                true,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
        ),
        (
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
        ),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(256),
                false,
                zinc_const::bitlength::BYTE * 2,
                true,
            ),
        ),
        (
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
        ),
    );
    assert_eq!(
        IntegerConstant::infer_literal_types(
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(8),
                false,
                zinc_const::bitlength::BYTE,
                true,
            ),
            &mut IntegerConstant::new(
                Location::default(),
                BigInt::from(65535),
                false,
                zinc_const::bitlength::BYTE * 2,
                true,
            ),
        ),
        (
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
            Some(Type::integer(
                Some(Location::default()),
                false,
                zinc_const::bitlength::BYTE * 2
            )),
        ),
    );
}

#[test]
fn error_integer_too_large_ordinar_constant() {
    let input = r#"
fn main() {
    let invalid = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                location: Location::new(3, 19),
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(zinc_const::panic::TEST_DATA_VALID),
                bitlength: zinc_const::bitlength::FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_integer_too_large_loop_for_bound() {
    let input = r#"
fn main() {
    for i in 0..0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff {}
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                location: Location::new(3, 17),
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(zinc_const::panic::TEST_DATA_VALID),
                bitlength: zinc_const::bitlength::FIELD,
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_integer_too_large_pattern_match() {
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
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::IntegerTooLarge {
                location: Location::new(5, 9),
                value: BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").expect(zinc_const::panic::TEST_DATA_VALID),
                bitlength: zinc_const::bitlength::FIELD,
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
    let value = 42 as u64 >= 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_equals_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value >= 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_equals_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value >= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreaterEquals {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 <= 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_equals_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value <= 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_equals_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value <= Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesserEquals {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 > 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value > 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_greater_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value > Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchGreater {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 < 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value < 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_lesser_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value < Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchLesser {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitor() {
    let input = r#"
fn main() {
    let value = 42 as u64 | 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitor_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value | 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitor_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value | Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseOr {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitxor() {
    let input = r#"
fn main() {
    let value = 42 as u64 ^ 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitxor_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value ^ 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitxor_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value ^ Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseXor {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitand() {
    let input = r#"
fn main() {
    let value = 42 as u64 & 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitand_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value & 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_bitand_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value & Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchBitwiseAnd {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 + 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_addition_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value + 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_addition_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value + Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchAddition {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 - 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_subtraction_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value - 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_subtraction_two_enumerations() {
    let input = r#"
enum One {
    Value = 64,
}

enum Two {
    Value = 42,
}

fn main() {
    let value = One::Value - Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchSubtraction {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 * 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_multiplication_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value * 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_multiplication_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 5,
}

fn main() {
    let value = One::Value * Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchMultiplication {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 / 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_division_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value / 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_division_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value / Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchDivision {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 42 as u64 % 64 as u128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                location: Location::new(3, 17),
                first: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 8).to_string(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE * 16).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_remainder_enumeration() {
    let input = r#"
enum Default {
    Value = 42,
}

fn main() {
    let value = Default::Value % 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                location: Location::new(7, 17),
                first: "enumeration Default".to_owned(),
                second: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_types_mismatch_remainder_two_enumerations() {
    let input = r#"
enum One {
    Value = 42,
}

enum Two {
    Value = 64,
}

fn main() {
    let value = One::Value % Two::Value;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::TypesMismatchRemainder {
                location: Location::new(11, 17),
                first: "enumeration One".to_owned(),
                second: "enumeration Two".to_owned(),
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
    let value = 168 << -2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
                location: Location::new(3, 25),
                found: IntegerConstant::new(
                    Location::new(3, 25),
                    BigInt::from(-2),
                    true,
                    zinc_const::bitlength::BYTE,
                    true,
                )
                .to_string(),
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
    let value = 42 >> -2;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
                location: Location::new(3, 24),
                found: IntegerConstant::new(
                    Location::new(3, 24),
                    BigInt::from(-2),
                    true,
                    zinc_const::bitlength::BYTE,
                    true,
                )
                .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_addition_signed_negative() {
    let input = r#"
fn main() {
    let value = -120 + (-50);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                location: Location::new(3, 18),
                value: BigInt::from(-170),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_addition_signed_positive() {
    let input = r#"
fn main() {
    let value = 42 as i8 + 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                location: Location::new(3, 17),
                value: BigInt::from(142),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_addition_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 + 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowAddition {
                location: Location::new(3, 17),
                value: BigInt::from(297),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_subtraction_signed_negative() {
    let input = r#"
fn main() {
    let value = -42 - 100 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                location: Location::new(3, 18),
                value: BigInt::from(-142),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_subtraction_signed_positive() {
    let input = r#"
fn main() {
    let value = (50 as i8) - (-100);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                location: Location::new(3, 18),
                value: BigInt::from(150),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_subtraction_unsigned_negative() {
    let input = r#"
fn main() {
    let value = 42 - 255;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowSubtraction {
                location: Location::new(3, 17),
                value: BigInt::from(-213),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_multiplication_signed_negative() {
    let input = r#"
fn main() {
    let value = -100 * (2 as i8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                location: Location::new(3, 18),
                value: BigInt::from(-200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_multiplication_signed_positive() {
    let input = r#"
fn main() {
    let value = 100 as i8 * 2 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                location: Location::new(3, 17),
                value: BigInt::from(200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_multiplication_unsigned_positive() {
    let input = r#"
fn main() {
    let value = 42 * 10;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowMultiplication {
                location: Location::new(3, 17),
                value: BigInt::from(420),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_division_signed_positive() {
    let input = r#"
fn main() {
    let value = -128 / (-1);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowDivision {
                location: Location::new(3, 18),
                value: BigInt::from(128),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_casting_signed_positive() {
    let input = r#"
fn main() {
    let value = 200 as i8;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                location: Location::new(3, 17),
                value: BigInt::from(200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_casting_unsigned_negative() {
    let input = r#"
fn main() {
    let value = (-100 as u8);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowCasting {
                location: Location::new(3, 19),
                value: BigInt::from(-100),
                r#type: Type::integer(
                    Some(Location::default()),
                    false,
                    zinc_const::bitlength::BYTE,
                )
                .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_negation_signed_positive() {
    let input = r#"
fn main() {
    let value = --128;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                location: Location::new(3, 19),
                value: BigInt::from(128),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_negation_unsigned_negative() {
    let input = r#"
fn main() {
    let value = -200;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::OverflowNegation {
                location: Location::new(3, 18),
                value: BigInt::from(-200),
                r#type: Type::integer(Some(Location::default()), true, zinc_const::bitlength::BYTE)
                    .to_string(),
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
    let value = 42 as field / 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldDivision {
                location: Location::new(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_remainder() {
    let input = r#"
fn main() {
    let value = 42 as field % 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldRemainder {
                location: Location::new(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitor() {
    let input = r#"
fn main() {
    let value = -42 | -1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::new(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitor() {
    let input = r#"
fn main() {
    let value = 42 as field | 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::new(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitxor() {
    let input = r#"
fn main() {
    let value = -42 ^ -1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::new(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitxor() {
    let input = r#"
fn main() {
    let value = 42 as field ^ 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::new(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitand() {
    let input = r#"
fn main() {
    let value = -42 & -1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::new(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitand() {
    let input = r#"
fn main() {
    let value = 42 as field & 1 as field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::new(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_shift_left() {
    let input = r#"
fn main() {
    let value = -42 << 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::new(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_shift_left() {
    let input = r#"
fn main() {
    let value = 42 as field << 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::new(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_shift_right() {
    let input = r#"
fn main() {
    let value = -42 >> 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::new(3, 18),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_shift_right() {
    let input = r#"
fn main() {
    let value = 42 as field >> 1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::new(3, 17),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_signed_bitwise_not() {
    let input = r#"
fn main() {
    let value = ~-42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenSignedBitwise {
                location: Location::new(3, 19),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_bitwise_not() {
    let input = r#"
fn main() {
    let value = ~(42 as field);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldBitwise {
                location: Location::new(3, 19),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_forbidden_field_negation() {
    let input = r#"
fn main() {
    let value = -(42 as field);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ForbiddenFieldNegation {
                location: Location::new(3, 19),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_zero_division() {
    let input = r#"
fn main() {
    let value = 42 / 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroDivision {
            location: Location::new(3, 22),
        })),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_zero_remainder() {
    let input = r#"
fn main() {
    let value = 42 % 0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        ElementError::Constant(ConstantError::Integer(
            IntegerConstantError::ZeroRemainder {
                location: Location::new(3, 22),
            },
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
