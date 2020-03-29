//!
//! The generator expression tuple operand builder.
//!

use crate::generator::expression::operand::tuple::Expression as TupleExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::semantic::element::r#type::Type as SemanticType;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    expressions: Vec<(Type, GeneratorExpression)>,
}

impl Builder {
    pub fn push_expression(&mut self, r#type: SemanticType, value: GeneratorExpression) {
        if let Some(r#type) = Type::try_from_semantic(&r#type) {
            self.expressions.push((r#type, value));
        }
    }

    pub fn finish(self) -> TupleExpression {
        TupleExpression::new(self.expressions)
    }
}
