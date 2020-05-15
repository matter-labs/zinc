//!
//! The type caster error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    CastingFromInvalidType { from: String, to: String },
    CastingToInvalidType { from: String, to: String },
}
