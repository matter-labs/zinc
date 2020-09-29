//!
//! The type casting.
//!

#[cfg(test)]
mod tests;

pub mod error;

use crate::semantic::element::r#type::Type;

use self::error::Error;

///
/// The casting object namespace.
///
pub struct Caster {}

impl Caster {
    ///
    /// Validates casting from one type to another.
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
    /// T -> T (no effect, no errors)
    ///
    /// `b1` and `b2` are bitlengths
    /// `T` is any type
    ///
    /// For more information on type semantics, see the official Zinc book.
    ///
    pub fn cast(from: &Type, to: &Type) -> Result<(), Error> {
        match (from, to) {
            (Type::IntegerUnsigned { .. }, Type::IntegerUnsigned { .. }) => Ok(()),
            (Type::IntegerUnsigned { .. }, Type::IntegerSigned { .. }) => Ok(()),
            (Type::IntegerUnsigned { .. }, Type::Field(_)) => Ok(()),
            (Type::IntegerSigned { .. }, Type::IntegerSigned { .. }) => Ok(()),
            (Type::IntegerSigned { .. }, Type::IntegerUnsigned { .. }) => Ok(()),
            (Type::IntegerSigned { .. }, Type::Field(_)) => Ok(()),
            (Type::Enumeration(_), Type::IntegerSigned { .. }) => Ok(()),
            (Type::Enumeration(_), Type::IntegerUnsigned { .. }) => Ok(()),
            (Type::Enumeration(_), Type::Field(_)) => Ok(()),
            (from, to) if from == to => Ok(()),

            (from @ Type::IntegerUnsigned { .. }, to) => Err(Error::CastingToInvalidType {
                from: from.to_string(),
                to: to.to_string(),
            }),
            (from @ Type::IntegerSigned { .. }, to) => Err(Error::CastingToInvalidType {
                from: from.to_string(),
                to: to.to_string(),
            }),
            (from, to) => Err(Error::CastingFromInvalidType {
                from: from.to_string(),
                to: to.to_string(),
            }),
        }
    }
}
