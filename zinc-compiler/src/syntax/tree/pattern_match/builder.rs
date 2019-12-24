//!
//! The match pattern builder.
//!

use crate::lexical::Location;
use crate::syntax::BooleanLiteral;
use crate::syntax::Expression;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchPattern;
use crate::syntax::MatchPatternVariant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    boolean_literal: Option<BooleanLiteral>,
    integer_literal: Option<IntegerLiteral>,
    binding: Option<Identifier>,
    path: Expression,
    is_wildcard: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_boolean_literal(&mut self, value: BooleanLiteral) {
        self.boolean_literal = Some(value);
    }

    pub fn set_integer_literal(&mut self, value: IntegerLiteral) {
        self.integer_literal = Some(value);
    }

    pub fn set_binding(&mut self, value: Identifier) {
        self.binding = Some(value);
    }

    pub fn extend_with_expression(&mut self, expression: Expression) {
        self.move_binding_to_path();
        self.path.elements.extend(expression);
    }

    pub fn push_path_operator(&mut self, location: Location, operator: ExpressionOperator) {
        self.move_binding_to_path();
        self.path.elements.push(ExpressionElement::new(
            location,
            ExpressionObject::Operator(operator),
        ));
    }

    pub fn set_wildcard(&mut self) {
        self.is_wildcard = true;
    }

    pub fn finish(mut self) -> MatchPattern {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let variant = if self.is_wildcard {
            MatchPatternVariant::Wildcard
        } else if let Some(boolean_literal) = self.boolean_literal.take() {
            MatchPatternVariant::BooleanLiteral(boolean_literal)
        } else if let Some(integer_literal) = self.integer_literal.take() {
            MatchPatternVariant::IntegerLiteral(integer_literal)
        } else if let Some(identifier) = self.binding.take() {
            MatchPatternVariant::Binding(identifier)
        } else if !self.path.elements.is_empty() {
            MatchPatternVariant::Path(self.path)
        } else {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "boolean | integer | binding | path | wildcard"
            );
        };

        MatchPattern::new(location, variant)
    }

    fn move_binding_to_path(&mut self) {
        if let Some(binding) = self.binding.take() {
            self.path.location = binding.location;
            self.path.elements.push(ExpressionElement::new(
                binding.location,
                ExpressionObject::Operand(ExpressionOperand::Identifier(binding)),
            ));
        }
    }
}
