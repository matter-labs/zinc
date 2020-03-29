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
    /// u<b1> -> u<b2>
    /// u<b1> -> i<b2>
    /// u<b1> -> field
    /// i<b1> -> i<b2>
    /// i<b1> -> u<b2>
    /// i<b1> -> field
    /// enum<b1> -> i<b2>
    /// enum<b1> -> u<b2>
    /// enum<b1> -> field
    /// x -> y, where x == y
    ///
    /// `b1` and `b2` are bitlengths
    /// `x` and `y` are types
    ///
    pub fn cast(from: &Type, to: &Type) -> Result<(), Error> {
        match (from, to) {
            (Type::IntegerUnsigned { .. }, Type::IntegerUnsigned { .. }) => Ok(()),
            (Type::IntegerUnsigned { .. }, Type::IntegerSigned { .. }) => Ok(()),
            (Type::IntegerUnsigned { .. }, Type::Field) => Ok(()),
            (from @ Type::IntegerUnsigned { .. }, to) => {
                Err(Error::casting_to_invalid_type(from, to))
            }
            (Type::IntegerSigned { .. }, Type::IntegerSigned { .. }) => Ok(()),
            (Type::IntegerSigned { .. }, Type::IntegerUnsigned { .. }) => Ok(()),
            (Type::IntegerSigned { .. }, Type::Field) => Ok(()),
            (from @ Type::IntegerSigned { .. }, to) => {
                Err(Error::casting_to_invalid_type(from, to))
            }
            (Type::Enumeration(_), Type::IntegerSigned { .. }) => Ok(()),
            (Type::Enumeration(_), Type::IntegerUnsigned { .. }) => Ok(()),
            (Type::Enumeration(_), Type::Field) => Ok(()),
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
