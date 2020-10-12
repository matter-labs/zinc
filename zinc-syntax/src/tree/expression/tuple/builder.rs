//!
//! The tuple expression builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::expression::tuple::Expression as TupleExpression;

///
/// The tuple expression builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The tuple expression inner element expressions.
    elements: Vec<ExpressionTree>,
    /// If the tuple has a comma after the first element.
    /// If the comma is present, the expression is definitely a tuple.
    /// If the comma is absent, the expression is an ordinar parenthesized expression.
    has_comma: bool,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_expression(&mut self, expression: ExpressionTree) {
        self.elements.push(expression);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_comma(&mut self) {
        self.has_comma = true;
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> ExpressionTree {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        if self.elements.is_empty() {
            ExpressionTree::new(
                location,
                ExpressionTreeNode::Operand(ExpressionOperand::LiteralUnit(location)),
            )
        } else if self.elements.len() > 1 {
            ExpressionTree::new(
                location,
                ExpressionTreeNode::Operand(ExpressionOperand::Tuple(TupleExpression::new(
                    location,
                    self.elements,
                ))),
            )
        } else if let Some(element) = self.elements.pop() {
            if self.has_comma {
                ExpressionTree::new(
                    location,
                    ExpressionTreeNode::Operand(ExpressionOperand::Tuple(TupleExpression::new(
                        location,
                        vec![element],
                    ))),
                )
            } else {
                element
            }
        } else {
            panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "element");
        }
    }
}
