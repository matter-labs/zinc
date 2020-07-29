//!
//! The semantic analyzer typed trait.
//!

use crate::semantic::element::r#type::Type;

///
/// Implemented by entities, which are typed and their types can be compared.
///
pub trait ITyped {
    ///
    /// Returns the entity type.
    ///
    fn r#type(&self) -> Type;

    ///
    /// Checks whether the entity's type is equals to the other's.
    ///
    fn has_the_same_type_as(&self, other: &Self) -> bool;
}
