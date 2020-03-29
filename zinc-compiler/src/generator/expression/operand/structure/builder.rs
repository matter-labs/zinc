//!
//! The generator expression structure operand builder.
//!

use crate::generator::expression::operand::structure::Expression as StructureExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::semantic::element::r#type::Type as SemanticType;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    fields: Vec<(String, Type, GeneratorExpression)>,
}

impl Builder {
    pub fn push_field(
        &mut self,
        name: String,
        r#type: SemanticType,
        expression: GeneratorExpression,
    ) {
        if let Some(r#type) = Type::try_from_semantic(&r#type) {
            self.fields.push((name, r#type, expression));
        }
    }

    pub fn finish(self) -> StructureExpression {
        StructureExpression::new(self.fields)
    }
}
