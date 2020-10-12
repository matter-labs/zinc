//!
//! The expression tree builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The expression tree builder.
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The value of the node, which is either an operand or operator.
    value: Option<ExpressionTreeNode>,
    /// The left child of the node.
    left: Option<ExpressionTree>,
    /// The right child of the node.
    right: Option<ExpressionTree>,
}

impl Builder {
    ///
    /// Pushes the subtree into a free branch.
    ///
    /*
    If no branches are occupied:

           Operator                          Operator
           /      \                          /      \
          /        \            ->          /        \
         /          \                      /          \
        X            X                   Node          X

    If the left branch is occupied:

           Operator                          Operator
           /     \                           /      \
          /       \            ->           /        \
         /         \                       /          \
      Node          X                   Node          Node

    If both branches are occupied:

           Operator                           Operator (same as below)
           /      \                           /      \
          /        \                         /        \
         /          \                       /          \
      Node         Node        ->      Operator        Node (new)
                                       /      \
                                      /        \
                                     /          \
                                   Node        Node
    */
    pub fn eat(&mut self, value: ExpressionTree) {
        if self.left.is_none() {
            self.set_left(value);
        } else if self.right.is_none() {
            self.set_right(value);
        } else {
            self.left = Some(self.to_owned().finish());
            self.set_location_if_unset(value.location);
            self.set_right(value);
        }
    }

    ///
    /// Puts the operand to the current node.
    ///
    /*
    If the node is not set:

           X                        Operand
          / \                        /   \
         /   \           ->         /     \
        /     \                    /       \
       X       X                  X         X

    If the node is set and no branches are occupied:

           Operator                          Operator
           /     \                          /        \
          /       \            ->          /          \
         /         \                      /            \
        X           X                  Operand          X

    If the node is set and the left branch is occupied:

           Operator                          Operator
           /      \                          /      \
          /        \           ->           /        \
         /          \                      /          \
      Operand        X                 Operand      Operand

    If the node is set and both branches are occupied:

           Operator                           Operator (same as below)
           /      \                          /        \
          /        \                        /          \
         /          \                      /            \
     Operand      Operand       ->     Operator      Operand (new)
                                       /      \
                                      /        \
                                     /          \
                                 Operand      Operand
    */
    pub fn eat_operand(&mut self, value: ExpressionOperand, location: Location) {
        if self.value.is_none() {
            self.set_location_if_unset(location);
            self.set_value_operand(value);
        } else if self.left.is_none() {
            self.set_left_operand(value, location);
        } else if self.right.is_none() {
            self.set_right_operand(value, location);
        } else {
            self.left = Some(self.to_owned().finish());
            self.set_location_if_unset(location);
            self.set_right_operand(value, location);
        }
    }

    ///
    /// Puts the operator to the current node.
    ///
    /*
    If the node is not set:

           X                        Operator
          / \                        /    \
         /   \          ->          /      \
        /     \                    /        \
       X       X                  X          X

    If the node is set:

           Operator                           Operator (same as below)
           /      \                          /        \
          /        \                        /          \
         /          \                      /            \
       Node        Node        ->      Operator          X
                                       /      \
                                      /        \
                                     /          \
                                   Node        Node
    */
    pub fn eat_operator(&mut self, value: ExpressionOperator, location: Location) {
        self.set_location_if_unset(location);
        if self.value.is_some() {
            self.left = Some(self.to_owned().finish());
            self.location = Some(location);
            self.right = None;
        }
        self.set_value_operator(value);
    }

    ///
    /// Checks whether the tree node and its leaves are empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.left.is_none() && self.right.is_none()
    }

    ///
    /// Finalizes the builder and returns the built expression tree.
    ///
    /// If the current node is empty, but the left leaf is set, the leaf is moved up
    /// to the current node.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> ExpressionTree {
        if self.value.is_none() && self.left.is_some() {
            return self.left.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "left")
            });
        }

        ExpressionTree::new_with_leaves(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.value.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "value")
            }),
            self.left.take(),
            self.right.take(),
        )
    }

    ///
    /// Sets the location.
    ///
    fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the location if it has not been set.
    ///
    fn set_location_if_unset(&mut self, value: Location) {
        if self.location.is_none() {
            self.set_location(value);
        }
    }

    ///
    /// Sets the current node value to the operand.
    ///
    fn set_value_operand(&mut self, value: ExpressionOperand) {
        self.value = Some(ExpressionTreeNode::operand(value));
    }

    ///
    /// Sets the current node value to the operator.
    ///
    fn set_value_operator(&mut self, value: ExpressionOperator) {
        self.value = Some(ExpressionTreeNode::operator(value));
    }

    ///
    /// Sets the current node left child.
    ///
    fn set_left(&mut self, value: ExpressionTree) {
        self.left = Some(value);
    }

    ///
    /// Sets the current node left child to the operand.
    ///
    fn set_left_operand(&mut self, value: ExpressionOperand, location: Location) {
        self.left = Some(ExpressionTree::new(
            location,
            ExpressionTreeNode::operand(value),
        ));
    }

    ///
    /// Sets the current node right child.
    ///
    fn set_right(&mut self, value: ExpressionTree) {
        self.right = Some(value);
    }

    ///
    /// Sets the current node right child to the operand.
    ///
    fn set_right_operand(&mut self, value: ExpressionOperand, location: Location) {
        self.right = Some(ExpressionTree::new(
            location,
            ExpressionTreeNode::operand(value),
        ));
    }
}
