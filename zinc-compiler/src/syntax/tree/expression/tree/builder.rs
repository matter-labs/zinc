//!
//! The expression tree builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    location: Option<Location>,
    value: Option<ExpressionTreeNode>,
    left: Option<ExpressionTree>,
    right: Option<ExpressionTree>,
}

impl Builder {
    pub fn eat(&mut self, value: ExpressionTree) {
        if self.left.is_none() {
            self.set_left(value);
        } else if self.right.is_none() {
            self.set_right(value);
        } else {
            self.left = Some(self.clone().finish());
            self.set_location_if_unset(value.location);
            self.right = None;
            self.set_right(value);
        }
    }

    pub fn eat_operand(&mut self, value: ExpressionOperand, location: Location) {
        if self.value.is_none() {
            self.set_location_if_unset(location);
            self.set_value_operand(value);
        } else if self.left.is_none() {
            self.set_left_operand(value, location);
        } else if self.right.is_none() {
            self.set_right_operand(value, location);
        } else {
            self.left = Some(self.clone().finish());
            self.set_location_if_unset(location);
            self.right = None;
            self.set_right_operand(value, location);
        }
    }

    pub fn eat_operator(&mut self, value: ExpressionOperator, location: Location) {
        self.set_location_if_unset(location);
        if self.value.is_some() {
            self.left = Some(self.clone().finish());
            self.location = Some(location);
            self.right = None;
        }
        self.set_value_operator(value);
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.left.is_none() && self.right.is_none()
    }

    pub fn finish(mut self) -> ExpressionTree {
        if self.value.is_none() && self.left.is_some() {
            return self
                .left
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "left"));
        }

        ExpressionTree::new(
            self.location
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location")),
            self.value
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "value")),
            self.left.take(),
            self.right.take(),
        )
    }

    fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    fn set_location_if_unset(&mut self, value: Location) {
        if self.location.is_none() {
            self.set_location(value);
        }
    }

    fn set_value_operand(&mut self, value: ExpressionOperand) {
        self.value = Some(ExpressionTreeNode::operand(value));
    }

    fn set_value_operator(&mut self, value: ExpressionOperator) {
        self.value = Some(ExpressionTreeNode::operator(value));
    }

    fn set_left(&mut self, value: ExpressionTree) {
        self.left = Some(value);
    }

    fn set_left_operand(&mut self, value: ExpressionOperand, location: Location) {
        self.left = Some(ExpressionTree::new(
            location,
            ExpressionTreeNode::operand(value),
            None,
            None,
        ));
    }

    fn set_right(&mut self, value: ExpressionTree) {
        self.right = Some(value);
    }

    fn set_right_operand(&mut self, value: ExpressionOperand, location: Location) {
        self.right = Some(ExpressionTree::new(
            location,
            ExpressionTreeNode::operand(value),
            None,
            None,
        ));
    }
}
