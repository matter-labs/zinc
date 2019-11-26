//!
//! Casting.
//!

mod error;

pub use self::error::Error;

use crate::syntax::TypeVariant;

///
/// Only the following casts are possible:
/// u(b1) -> u(b2), b1 <= b2
/// u(b1) -> i(b2), b1 <= b2
/// u(b1) -> field
/// i(b1) -> i(b2), b1 <= b2
/// i(b1) -> u(b2), b1 <= b2
/// i(b1) -> field
/// x -> y, x == y
///
/// `b1` and `b2` are bitlengths
/// `a` and `b` are types
///
pub fn validate(from: &TypeVariant, to: &TypeVariant) -> Result<(), Error> {
    match (from, to) {
        (
            TypeVariant::IntegerUnsigned { bitlength: b1 },
            TypeVariant::IntegerUnsigned { bitlength: b2 },
        ) => {
            let (b1, b2) = (*b1, *b2);
            if b1 >= crate::BITLENGTH_FIELD_PADDED - crate::BITLENGTH_BYTE || b1 > b2 {
                Err(Error::DataLossPossible(b1, b2))
            } else {
                Ok(())
            }
        }
        (
            TypeVariant::IntegerUnsigned { bitlength: b1 },
            TypeVariant::IntegerSigned { bitlength: b2 },
        ) => {
            let (b1, b2) = (*b1, *b2);
            if b1 >= crate::BITLENGTH_FIELD_PADDED - crate::BITLENGTH_BYTE || b1 > b2 {
                Err(Error::DataLossPossible(b1, b2))
            } else {
                Ok(())
            }
        }
        (TypeVariant::IntegerUnsigned { .. }, TypeVariant::Field) => Ok(()),
        (from @ TypeVariant::IntegerUnsigned { .. }, to) => {
            Err(Error::ToInvalidType(from.to_owned(), to.to_owned()))
        }
        (
            TypeVariant::IntegerSigned { bitlength: b1 },
            TypeVariant::IntegerSigned { bitlength: b2 },
        ) => {
            let (b1, b2) = (*b1, *b2);
            if b1 >= crate::BITLENGTH_FIELD_PADDED - crate::BITLENGTH_BYTE || b1 > b2 {
                Err(Error::DataLossPossible(b1, b2))
            } else {
                Ok(())
            }
        }
        (
            TypeVariant::IntegerSigned { bitlength: b1 },
            TypeVariant::IntegerUnsigned { bitlength: b2 },
        ) => {
            let (b1, b2) = (*b1, *b2);
            if b1 >= crate::BITLENGTH_FIELD_PADDED - crate::BITLENGTH_BYTE || b1 > b2 {
                Err(Error::DataLossPossible(b1, b2))
            } else {
                Ok(())
            }
        }
        (TypeVariant::IntegerSigned { .. }, TypeVariant::Field) => Ok(()),
        (from @ TypeVariant::IntegerSigned { .. }, to) => {
            Err(Error::ToInvalidType(from.to_owned(), to.to_owned()))
        }
        (from, to) => {
            if from == to {
                Ok(())
            } else {
                Err(Error::FromInvalidType(from.to_owned(), to.to_owned()))
            }
        }
    }
}
