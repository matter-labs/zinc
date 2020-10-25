//!
//! The binding.
//!

pub mod builder;

use crate::tree::pattern_binding::Pattern as BindingPattern;
use crate::tree::r#type::Type;

use zinc_lexical::Location;

///
/// The binding.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    /// The location of the syntax construction.
    pub location: Location,
    /// The binding.
    pub pattern: BindingPattern,
    /// The optional binding type.
    pub r#type: Option<Type>,
}

impl Binding {
    ///
    /// Creates a binding pattern.
    ///
    pub fn new(location: Location, pattern: BindingPattern, r#type: Option<Type>) -> Self {
        Self {
            location,
            pattern,
            r#type,
        }
    }
}
