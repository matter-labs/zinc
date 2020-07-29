//!
//! The type caster error.
//!

///
/// The type caster error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// When the left operand cannot be casted to anything.
    CastingFromInvalidType {
        /// The first operand item type.
        from: String,
        /// The second operand type.
        to: String,
    },
    /// When the left operand cannot be casted to the right operand type.
    CastingToInvalidType {
        /// The first operand item type.
        from: String,
        /// The second operand type.
        to: String,
    },
}
