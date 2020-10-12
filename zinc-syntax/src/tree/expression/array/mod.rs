//!
//! The array expression.
//!

pub mod builder;
pub mod variant;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;

use self::variant::Variant;

///
/// The array expression.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// The location of the syntax construction.
    pub location: Location,
    /// The array literal variant.
    pub variant: Variant,
}

impl Expression {
    ///
    /// Creates an array expression with separate values.
    ///
    pub fn new_list(location: Location, elements: Vec<ExpressionTree>) -> Self {
        Self {
            location,
            variant: Variant::new_list(elements),
        }
    }

    ///
    /// Creates an array expression with a single repeated value.
    ///
    pub fn new_repeated(
        location: Location,
        expression: ExpressionTree,
        size_expression: ExpressionTree,
    ) -> Self {
        Self {
            location,
            variant: Variant::new_repeated(expression, size_expression),
        }
    }
}
