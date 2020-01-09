//!
//! The extern fn statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::BindingPattern;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct ExternFn {
    pub location: Location,
    pub identifier: Identifier,
    pub argument_bindings: Vec<BindingPattern>,
    pub return_type: Type,
}

impl ExternFn {
    pub fn new(
        location: Location,
        identifier: Identifier,
        argument_bindings: Vec<BindingPattern>,
        return_type: Type,
    ) -> Self {
        Self {
            location,
            identifier,
            argument_bindings,
            return_type,
        }
    }
}
