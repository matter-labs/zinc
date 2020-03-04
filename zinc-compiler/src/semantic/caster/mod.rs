//!
//! The type caster.
//!

mod tests;

pub mod error;

use crate::semantic::element::r#type::Type;

use self::error::Error;

pub struct Caster {}

impl Caster {
    ///
    /// Only the following casts are allowed:
    /// u<b1> -> u<b2>, where b1 <= b2
    /// u<b1> -> i<b2>, where b1 <= b2
    /// u<b1> -> field
    /// i<b1> -> i<b2>, where b1 <= b2
    /// i<b1> -> u<b2>, where b1 <= b2
    /// i<b1> -> field
    /// enum<b1> -> u<b2>, where b1 <= b2
    /// enum<b1> -> field
    /// x -> y, where x == y
    ///
    /// `b1` and `b2` are bitlengths
    /// `x` and `y` are types
    ///
    pub fn cast(from: &Type, to: &Type) -> Result<(), Error> {
        match (from, to) {
            (Type::IntegerUnsigned { bitlength: b1 }, Type::IntegerUnsigned { bitlength: b2 }) => {
                let (b1, b2) = (*b1, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::casting_integer_to_lesser_bitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerUnsigned { bitlength: b1 }, Type::IntegerSigned { bitlength: b2 }) => {
                let (b1, b2) = (*b1, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::casting_integer_to_lesser_bitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerUnsigned { .. }, Type::Field) => Ok(()),
            (from @ Type::IntegerUnsigned { .. }, to) => {
                Err(Error::casting_to_invalid_type(from, to))
            }
            (Type::IntegerSigned { bitlength: b1 }, Type::IntegerSigned { bitlength: b2 }) => {
                let (b1, b2) = (*b1, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::casting_integer_to_lesser_bitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerSigned { bitlength: b1 }, Type::IntegerUnsigned { bitlength: b2 }) => {
                let (b1, b2) = (*b1, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::casting_integer_to_lesser_bitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerSigned { .. }, Type::Field) => Ok(()),
            (Type::Enumeration(enumeration), Type::IntegerUnsigned { bitlength: b2 }) => {
                let (b1, b2) = (enumeration.bitlength, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::casting_integer_to_lesser_bitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::Enumeration(_enumeration), Type::Field) => Ok(()),
            (from @ Type::IntegerSigned { .. }, to) => {
                Err(Error::casting_to_invalid_type(from, to))
            }
            (from, to) => {
                if from == to {
                    Ok(())
                } else {
                    Err(Error::casting_from_invalid_type(from, to))
                }
            }
        }
    }
}
