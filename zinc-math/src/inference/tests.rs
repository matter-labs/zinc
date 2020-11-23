//!
//! The integer type inference tests.
//!

use std::str::FromStr;

use num::BigInt;

use crate::inference;
use crate::inference::r#type::Type as InferredType;
use crate::inference::result::Binary as BinaryInferenceResult;

#[test]
fn ok_minimal_bitlength() {
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("0").unwrap_or_default(), false,),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("255").unwrap_or_default(), false,),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("256").unwrap_or_default(), false,),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("65535").unwrap_or_default(), false,),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("65536").unwrap_or_default(), false,),
        Ok(zinc_const::bitlength::BYTE * 3),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("4294967295").unwrap_or_default(), false,),
        Ok(zinc_const::bitlength::BYTE * 4),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("4294967296").unwrap_or_default(), false,),
        Ok(zinc_const::bitlength::BYTE * 5),
    );
    assert_eq!(
        inference::minimal_bitlength(
            &BigInt::from_str("18446744073709551615").unwrap_or_default(),
            false,
        ),
        Ok(zinc_const::bitlength::BYTE * 8),
    );
    assert_eq!(
        inference::minimal_bitlength(
            &BigInt::from_str("18446744073709551616").unwrap_or_default(),
            false,
        ),
        Ok(zinc_const::bitlength::BYTE * 9),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("-128").unwrap_or_default(), true),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("127").unwrap_or_default(), true),
        Ok(zinc_const::bitlength::BYTE * 1),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("128").unwrap_or_default(), true),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("32767").unwrap_or_default(), true),
        Ok(zinc_const::bitlength::BYTE * 2),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("32768").unwrap_or_default(), true),
        Ok(zinc_const::bitlength::BYTE * 3),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("2147483647").unwrap_or_default(), true),
        Ok(zinc_const::bitlength::BYTE * 4),
    );
    assert_eq!(
        inference::minimal_bitlength(&BigInt::from_str("2147483648").unwrap_or_default(), true),
        Ok(zinc_const::bitlength::BYTE * 5),
    );
    assert_eq!(
        inference::minimal_bitlength(
            &BigInt::from_str("9223372036854775807").unwrap_or_default(),
            true,
        ),
        Ok(zinc_const::bitlength::BYTE * 8),
    );
    assert_eq!(
        inference::minimal_bitlength(
            &BigInt::from_str("9223372036854775808").unwrap_or_default(),
            true,
        ),
        Ok(zinc_const::bitlength::BYTE * 9),
    );
}

#[test]
#[allow(const_item_mutation)]
fn ok_literal_types() {
    // none of the operands are literals
    assert_eq!(
        inference::literal_types(
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
            false,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
        ),
        BinaryInferenceResult::none(),
    );
    assert_eq!(
        inference::literal_types(
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
            false,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
        ),
        BinaryInferenceResult::none(),
    );
    assert_eq!(
        inference::literal_types(
            false,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
        ),
        BinaryInferenceResult::none(),
    );
    assert_eq!(
        inference::literal_types(
            false,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
        ),
        BinaryInferenceResult::none(),
    );

    // the first operand is a literal
    assert_eq!(
        inference::literal_types(
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
        ),
        BinaryInferenceResult::first(InferredType::new(false, zinc_const::bitlength::BYTE * 2)),
    );
    assert_eq!(
        inference::literal_types(
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
        ),
        BinaryInferenceResult::first(InferredType::new(false, zinc_const::bitlength::BYTE * 2)),
    );

    // the second operand is a literal
    assert_eq!(
        inference::literal_types(
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
        ),
        BinaryInferenceResult::second(InferredType::new(false, zinc_const::bitlength::BYTE * 2)),
    );
    assert_eq!(
        inference::literal_types(
            false,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
        ),
        BinaryInferenceResult::second(InferredType::new(false, zinc_const::bitlength::BYTE * 2)),
    );

    // both operands are literals
    assert_eq!(
        inference::literal_types(
            true,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
        ),
        BinaryInferenceResult::both(
            InferredType::new(false, zinc_const::bitlength::BYTE * 2),
            InferredType::new(false, zinc_const::bitlength::BYTE * 2)
        ),
    );
    assert_eq!(
        inference::literal_types(
            true,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
        ),
        BinaryInferenceResult::both(
            InferredType::new(false, zinc_const::bitlength::BYTE * 2),
            InferredType::new(false, zinc_const::bitlength::BYTE * 2)
        ),
    );
    assert_eq!(
        inference::literal_types(
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
            true,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
        ),
        BinaryInferenceResult::both(
            InferredType::new(false, zinc_const::bitlength::BYTE * 2),
            InferredType::new(false, zinc_const::bitlength::BYTE * 2)
        ),
    );
    assert_eq!(
        inference::literal_types(
            true,
            &mut false,
            &mut zinc_const::bitlength::BYTE,
            true,
            &mut false,
            &mut (zinc_const::bitlength::BYTE * 2),
        ),
        BinaryInferenceResult::both(
            InferredType::new(false, zinc_const::bitlength::BYTE * 2),
            InferredType::new(false, zinc_const::bitlength::BYTE * 2)
        ),
    );
}
