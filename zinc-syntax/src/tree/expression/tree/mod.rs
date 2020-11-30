//!
//! The expression tree.
//!

pub mod builder;
pub mod node;

use std::fmt;

use zinc_lexical::Location;

use self::node::Node;

///
/// The expression tree, where each node is either an operand or operator.
///
/// The highest operator precedence nodes are located deeper within a tree,
/// whereas the lowest ones are located at the top.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    /// The location of the syntax construction.
    pub location: Location,
    /// The value of the node, which is either an operand or operator.
    pub value: Box<Node>,
    /// The left child of the node.
    pub left: Option<Box<Self>>,
    /// The right child of the node.
    pub right: Option<Box<Self>>,
}

impl Tree {
    ///
    /// Initializes a tree with a single node at the top.
    ///
    pub fn new(location: Location, value: Node) -> Self {
        Self {
            location,
            value: Box::new(value),
            left: None,
            right: None,
        }
    }

    ///
    /// Initializes a tree with left and right leaves.
    ///
    pub fn new_with_leaves(
        location: Location,
        value: Node,
        left: Option<Self>,
        right: Option<Self>,
    ) -> Self {
        Self {
            location,
            value: Box::new(value),
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    ///
    /// Checks if the tree is a single block, conditional, or match expression.
    ///
    /// Is used to allow not terminating such expression with a semicolon.
    ///
    pub fn can_be_unterminated(&self) -> bool {
        match *self.value {
            Node::Operand(ref operand) => operand.can_be_unterminated(),
            _ => false,
        }
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(left), Some(right)) => write!(f, "{}{}{}", left, self.value, right),
            (Some(left), None) => write!(f, "{}{}", left, self.value),
            (None, Some(right)) => write!(f, "{}{}", self.value, right),
            (None, None) => write!(f, "{}", self.value),
        }
    }
}
