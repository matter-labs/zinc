//!
//! The integer type inference.
//!

#[cfg(test)]
mod tests;

pub mod result;
pub mod r#type;

use std::cmp;

use num::BigInt;
use num::Signed;

use crate::error::Error;

use self::r#type::Type;
use self::result::Binary as BinaryResult;

///
/// Infers the minimal bitlength enough to represent the `value` with sign specified
/// as `is_signed`.
///
pub fn minimal_bitlength(value: &BigInt, is_signed: bool) -> crate::Result<usize> {
    let mut bitlength = zinc_const::bitlength::BYTE;
    let mut exponent = BigInt::from(1 << zinc_const::bitlength::BYTE);

    while if is_signed {
        if value.is_negative() {
            let bound = -(exponent.clone() / BigInt::from(2));
            value < &bound
        } else {
            let bound = exponent.clone() / BigInt::from(2);
            value >= &bound
        }
    } else {
        value >= &exponent
    } {
        if bitlength == zinc_const::bitlength::INTEGER_MAX {
            exponent <<= zinc_const::bitlength::FIELD - zinc_const::bitlength::INTEGER_MAX;
            bitlength += zinc_const::bitlength::FIELD - zinc_const::bitlength::INTEGER_MAX;
        } else if bitlength == zinc_const::bitlength::FIELD {
            return Err(Error::Overflow {
                value: value.to_owned(),
                is_signed,
                bitlength: zinc_const::bitlength::FIELD,
            });
        } else {
            exponent <<= zinc_const::bitlength::BYTE;
            bitlength += zinc_const::bitlength::BYTE;
        }
    }

    if value.is_negative() && !is_signed {
        return Err(Error::Overflow {
            value: value.to_owned(),
            is_signed,
            bitlength: zinc_const::bitlength::FIELD,
        });
    }

    Ok(bitlength)
}

///
/// Infers the integer literal types.
///
/// If one of the operands is a literal, it inherits the other's operand type.
///
/// If both of the operands are literals, the smallest type enough to fit them is inferred.
///
pub fn literal_types(
    operand_1_is_literal: bool,
    operand_1_is_signed: &mut bool,
    operand_1_bitlength: &mut usize,
    operand_2_is_literal: bool,
    operand_2_is_signed: &mut bool,
    operand_2_bitlength: &mut usize,
) -> BinaryResult {
    if operand_1_is_literal && !operand_2_is_literal {
        *operand_1_is_signed = *operand_2_is_signed;
        *operand_1_bitlength = *operand_2_bitlength;

        BinaryResult::first(Type::new(*operand_1_is_signed, *operand_1_bitlength))
    } else if !operand_1_is_literal && operand_2_is_literal {
        *operand_2_is_signed = *operand_1_is_signed;
        *operand_2_bitlength = *operand_1_bitlength;

        BinaryResult::second(Type::new(*operand_2_is_signed, *operand_2_bitlength))
    } else if operand_1_is_literal && operand_2_is_literal {
        *operand_1_is_signed = *operand_1_is_signed || *operand_2_is_signed;

        *operand_1_bitlength = cmp::max(*operand_1_bitlength, *operand_2_bitlength);
        *operand_2_bitlength = *operand_1_bitlength;

        BinaryResult::both(
            Type::new(*operand_1_is_signed, *operand_1_bitlength),
            Type::new(*operand_2_is_signed, *operand_2_bitlength),
        )
    } else {
        BinaryResult::none()
    }
}
