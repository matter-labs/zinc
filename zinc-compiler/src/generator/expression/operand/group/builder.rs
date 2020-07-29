//!
//! The generator expression group operand builder.
//!

use crate::generator::expression::operand::group::Expression as GroupExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The generator expression group operand builder.
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// The typed group element expressions.
    expressions: Vec<(Type, GeneratorExpression)>,
}

impl Builder {
    ///
    /// Pushes a group expression.
    ///
    pub fn push_expression(&mut self, r#type: SemanticType, value: GeneratorExpression) {
        if let Some(r#type) = Type::try_from_semantic(&r#type) {
            self.expressions.push((r#type, value));
        }
    }

    ///
    /// Finilizes the builder and returns the built item.
    ///
    pub fn finish(self) -> GroupExpression {
        GroupExpression::new(self.expressions)
    }
}
