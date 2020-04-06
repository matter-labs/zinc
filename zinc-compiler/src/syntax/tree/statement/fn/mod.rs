//!
//! The fn statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub argument_bindings: Vec<BindingPattern>,
    pub return_type: Option<Type>,
    pub body: BlockExpression,
}

impl Statement {
    pub fn new(
        location: Location,
        identifier: Identifier,
        argument_bindings: Vec<BindingPattern>,
        return_type: Option<Type>,
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
