//!
//! The match pattern builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::pattern_match::variant::Variant as MatchPatternVariant;
use crate::syntax::tree::pattern_match::Pattern as MatchPattern;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    boolean_literal: Option<BooleanLiteral>,
    integer_literal: Option<IntegerLiteral>,
    binding: Option<Identifier>,
    path_builder: ExpressionTreeBuilder,
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

    pub fn push_path_operator(&mut self, operator: ExpressionOperator, location: Location) {
        self.move_binding_to_path();
        self.path_builder.eat_operator(operator, location);
    }

    pub fn push_path_element(&mut self, tree: ExpressionTree) {
        self.move_binding_to_path();
        self.path_builder.eat(tree);
    }

    pub fn set_is_wildcard(&mut self) {
        self.is_wildcard = true;
    }

    pub fn finish(mut self) -> MatchPattern {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location"));

        let variant = if self.is_wildcard {
            MatchPatternVariant::Wildcard
        } else if let Some(boolean_literal) = self.boolean_literal.take() {
            MatchPatternVariant::BooleanLiteral(boolean_literal)
        } else if let Some(integer_literal) = self.integer_literal.take() {
            MatchPatternVariant::IntegerLiteral(integer_literal)
        } else if let Some(identifier) = self.binding.take() {
            MatchPatternVariant::Binding(identifier)
        } else if !self.path_builder.is_empty() {
            MatchPatternVariant::Path(self.path_builder.finish())
        } else {
            panic!(
                "{}{}",
                crate::PANIC_BUILDER_REQUIRES_VALUE,
                "boolean | integer | binding | path | wildcard"
            );
        };

        MatchPattern::new(location, variant)
    }

    fn move_binding_to_path(&mut self) {
        if let Some(binding) = self.binding.take() {
            let location = binding.location;
            self.path_builder
                .eat_operand(ExpressionOperand::Identifier(binding), location);
        }
    }
}
