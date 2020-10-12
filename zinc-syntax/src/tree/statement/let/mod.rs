//!
//! The `let` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;
use crate::tree::r#type::Type;

///
/// The `let` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The variable identifier.
    pub identifier: Identifier,
    /// If the variable is mutable.
    pub is_mutable: bool,
    /// The optional variable type, which is inferred otherwise.
    pub r#type: Option<Type>,
    /// The expression assigned to the variable.
    pub expression: ExpressionTree,
}

impl Statement {
    ///
    /// Creates a `let` statement.
    ///
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_mutable: bool,
        r#type: Option<Type>,
        expression: ExpressionTree,
    ) -> Self {
        Self {
            location,
            identifier,
            is_mutable,
            r#type,
            expression,
        }
    }
}
