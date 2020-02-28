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
                    Err(Error::ToLesserBitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerUnsigned { bitlength: b1 }, Type::IntegerSigned { bitlength: b2 }) => {
                let (b1, b2) = (*b1, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::ToLesserBitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerUnsigned { .. }, Type::Field) => Ok(()),
            (from @ Type::IntegerUnsigned { .. }, to) => {
                Err(Error::ToInvalidType(from.to_string(), to.to_string()))
            }
            (Type::IntegerSigned { bitlength: b1 }, Type::IntegerSigned { bitlength: b2 }) => {
                let (b1, b2) = (*b1, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::ToLesserBitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerSigned { bitlength: b1 }, Type::IntegerUnsigned { bitlength: b2 }) => {
                let (b1, b2) = (*b1, *b2);
                if b1 > crate::BITLENGTH_MAX_INT || b1 > b2 {
                    Err(Error::ToLesserBitlength(b1, b2))
                } else {
                    Ok(())
                }
            }
            (Type::IntegerSigned { .. }, Type::Field) => Ok(()),
            (from @ Type::IntegerSigned { .. }, to) => {
                Err(Error::ToInvalidType(from.to_string(), to.to_string()))
            }
            (from, to) => {
                if from == to {
                    Ok(())
                } else {
                    Err(Error::FromInvalidType(from.to_string(), to.to_string()))
                }
            }
        }
    }
}
