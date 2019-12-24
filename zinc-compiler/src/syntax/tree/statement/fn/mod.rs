//!
//! The fn statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::BindingPattern;
use crate::syntax::BlockExpression;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Fn {
    pub location: Location,
    pub identifier: Identifier,
    pub argument_bindings: Vec<BindingPattern>,
    pub return_type: Type,
    pub body: BlockExpression,
}

impl Fn {
    pub fn new(
        location: Location,
        identifier: Identifier,
        argument_bindings: Vec<BindingPattern>,
        return_type: Type,
        body: BlockExpression,
    ) -> Self {
        Self {
            location,
            identifier,
            argument_bindings,
            return_type,
            body,
        }
    }
}
