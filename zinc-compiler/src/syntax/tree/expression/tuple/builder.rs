//!
//! The tuple expression builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::expression::tuple::Expression as TupleExpression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    elements: Vec<ExpressionTree>,
    has_comma: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn push_expression(&mut self, expression: ExpressionTree) {
        self.elements.push(expression);
    }

    pub fn set_comma(&mut self) {
        self.has_comma = true;
    }

    pub fn finish(mut self) -> ExpressionTree {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location"));

        if self.elements.is_empty() {
            ExpressionTree::new(
                location,
                ExpressionTreeNode::Operand(ExpressionOperand::LiteralUnit),
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
            panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "element");
        }
    }
}
