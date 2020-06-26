//!
//! The `fn` statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::attribute::Attribute;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
use crate::syntax::tree::r#type::Type;

///
/// The `fn` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// If the function is public.
    pub is_public: bool,
    /// If the function is constant.
    pub is_constant: bool,
    /// The function identifier.
    pub identifier: Identifier,
    /// The function argument bindings.
    pub argument_bindings: Vec<BindingPattern>,
    /// The optional function return type, which is `()` if not specified.
    pub return_type: Option<Type>,
    /// The function block.
    pub body: BlockExpression,
    /// The function outer attributes.
    pub attributes: Vec<Attribute>,
}

impl Statement {
    ///
    /// Creates an `fn` statement.
    ///
    #[allow(clippy::too_many_arguments)]
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
