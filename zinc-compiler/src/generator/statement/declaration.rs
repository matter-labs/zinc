//!
//! The generator declaration statement.
//!

use crate::generator::expression::Expression;
use crate::generator::r#type::Type;
use crate::semantic::Type as SemanticType;

#[derive(Debug, Clone)]
pub struct Statement {
    pub r#type: Option<Type>,
    pub expression: Expression,
}

impl Statement {
    pub fn new(r#type: SemanticType, expression: Expression) -> Self {
        Self {
            r#type: Type::try_from_semantic(&r#type),
            expression,
        }
    }
}
