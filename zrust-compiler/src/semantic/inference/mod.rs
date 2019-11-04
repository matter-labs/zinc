//!
//! Inference.
//!

mod error;

pub use self::error::Error;

use num_bigint::BigInt;
use num_traits::Num;

use crate::lexical::IntegerLiteral;

pub fn integer_literal(literal: &IntegerLiteral) -> Result<(BigInt, usize), Error> {
    let (string, base) = match literal {
        IntegerLiteral::Decimal { value } => (value, crate::BASE_DECIMAL as u32),
        IntegerLiteral::Hexadecimal { value } => (value, crate::BASE_HEXADECIMAL as u32),
    };

    let number = BigInt::from_str_radix(string, base).expect("Always valid");
    let mut bitlength = crate::BITLENGTH_BYTE;
    let mut exponent = BigInt::from(crate::MAX_VALUE_BYTE);
    while number >= exponent {
        if bitlength == crate::BITLENGTH_MAX_INT {
            exponent *= crate::MAX_VALUE_BYTE / 2 / 2;
            bitlength += crate::BITLENGTH_FIELD - crate::BITLENGTH_MAX_INT;
        } else if bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::LiteralTooLarge(literal.to_owned(), bitlength));
        } else {
            exponent *= crate::MAX_VALUE_BYTE;
            bitlength += crate::BITLENGTH_BYTE;
        }
    }

    Ok((number, bitlength))
}

//pub fn enough_bitlength(literals: &[&IntegerLiteral]) -> Result<usize, Error> {
//    let mut max = 0;
//    for literal in literals.iter() {
//        let bitlength = integer_literal(literal)?.1;
//        if bitlength > max {
//            max = bitlength;
//        }
//    }
//    Ok(max)
//}
