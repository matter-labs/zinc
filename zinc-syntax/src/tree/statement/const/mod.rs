//!
//! The `const` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;
use crate::tree::r#type::Type;

///
/// The `const` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The constant identifier.
    pub identifier: Identifier,
    /// The constant type.
    pub r#type: Type,
    /// The expression assigned to the constant.
    pub expression: ExpressionTree,
}

impl Statement {
    ///
    /// Creates a `const` statement.
    ///
    pub fn new(
        location: Location,
        identifier: Identifier,
        r#type: Type,
        expression: ExpressionTree,
    ) -> Self {
        Self {
            location,
            identifier,
            r#type,
            expression,
        }
    }
}
