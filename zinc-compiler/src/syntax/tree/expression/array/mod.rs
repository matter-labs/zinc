//!
//! The array expression.
//!

pub mod builder;
pub mod variant;

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

use self::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub variant: Variant,
}

impl Expression {
    pub fn new_list(location: Location, elements: Vec<ExpressionTree>) -> Self {
        Self {
            location,
            variant: Variant::new_list(elements),
        }
    }

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
