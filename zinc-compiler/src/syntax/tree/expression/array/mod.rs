//!
//! The array expression.
//!

pub mod builder;
pub mod variant;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression as SyntaxExpression;

use self::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub variant: Variant,
}

impl Expression {
    pub fn new_list(location: Location, elements: Vec<SyntaxExpression>) -> Self {
        Self {
            location,
            variant: Variant::new_list(elements),
        }
    }

    pub fn new_repeated(
        location: Location,
        expression: SyntaxExpression,
        size_expression: SyntaxExpression,
    ) -> Self {
        Self {
            location,
            variant: Variant::new_repeated(expression, size_expression),
        }
    }
}
