//!
//! The fn statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::attribute::Attribute;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub is_public: bool,
    pub is_constant: bool,
    pub identifier: Identifier,
    pub argument_bindings: Vec<BindingPattern>,
    pub return_type: Option<Type>,
    pub body: BlockExpression,
    pub attributes: Vec<Attribute>,
}

impl Statement {
    pub fn new(
        location: Location,
        is_public: bool,
        is_constant: bool,
        identifier: Identifier,
        argument_bindings: Vec<BindingPattern>,
        return_type: Option<Type>,
        body: BlockExpression,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            location,
            is_public,
            is_constant,
            identifier,
            argument_bindings,
            return_type,
            body,
            attributes,
        }
    }
}
