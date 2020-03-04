//!
//! The type caster error.
//!

use crate::semantic::element::r#type::Type;

#[derive(Debug, PartialEq)]
pub enum Error {
    CastingFromInvalidType { from: String, to: String },
    CastingToInvalidType { from: String, to: String },
    CastingIntegerToLesserBitlength { from: usize, to: usize },
}

impl Error {
    pub fn casting_from_invalid_type(from: &Type, to: &Type) -> Self {
        Self::CastingFromInvalidType {
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    pub fn casting_to_invalid_type(from: &Type, to: &Type) -> Self {
        Self::CastingToInvalidType {
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    pub fn casting_integer_to_lesser_bitlength(from: usize, to: usize) -> Self {
        Self::CastingIntegerToLesserBitlength { from, to }
    }
}
