//!
//! The Zinc VM template value error.
//!

///
/// The error location propagating trait.
///
pub trait IContext {
    ///
    /// Propagates the error location in an array.
    ///
    fn push_array(self, index: usize) -> Self;

    ///
    /// Propagates the error location in a structure.
    ///
    fn push_structure(self, name: &str) -> Self;
}
