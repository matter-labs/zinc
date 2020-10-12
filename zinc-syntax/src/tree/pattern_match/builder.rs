//!
//! The match pattern builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;
use crate::tree::literal::boolean::Literal as BooleanLiteral;
use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::pattern_match::variant::Variant as MatchPatternVariant;
use crate::tree::pattern_match::Pattern as MatchPattern;

///
/// The match pattern builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The boolean literal variant, which means that the pattern is a boolean constant.
    boolean_literal: Option<BooleanLiteral>,
    /// The integer literal variant, which means that the pattern is an integer constant.
    integer_literal: Option<IntegerLiteral>,
    /// The binding variant, which means that the pattern is a variable binding.
    binding: Option<Identifier>,
    /// The path builder variant, which means that the pattern is a path expression.
    path_builder: ExpressionTreeBuilder,
    /// If the pattern variant is a wildcard.
    is_wildcard: bool,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_boolean_literal(&mut self, value: BooleanLiteral) {
        self.boolean_literal = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_integer_literal(&mut self, value: IntegerLiteral) {
        self.integer_literal = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_binding(&mut self, value: Identifier) {
        self.binding = Some(value);
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_path_operator(&mut self, operator: ExpressionOperator, location: Location) {
        self.move_binding_to_path();
        self.path_builder.eat_operator(operator, location);
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_path_element(&mut self, tree: ExpressionTree) {
        self.move_binding_to_path();
        self.path_builder.eat(tree);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_wildcard(&mut self) {
        self.is_wildcard = true;
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> MatchPattern {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
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
        } else if !self.path_builder.is_empty() {
            MatchPatternVariant::Path(self.path_builder.finish())
        } else {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "boolean | integer | binding | path | wildcard"
            );
        };

        MatchPattern::new(location, variant)
    }

    ///
    /// Convert a binding with a single element to a path expression, which can consist of
    /// more than one element.
    ///
    fn move_binding_to_path(&mut self) {
        if let Some(binding) = self.binding.take() {
            let location = binding.location;
            self.path_builder
                .eat_operand(ExpressionOperand::Identifier(binding), location);
        }
    }
}
