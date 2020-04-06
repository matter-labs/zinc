//!
//! The expression tree.
//!

pub mod builder;
pub mod node;

use crate::lexical::token::location::Location;

use self::node::Node;

///
/// The expression tree, where each node is either an operand or operator.
///
/// The highest operator precedence nodes are located deeper within a tree,
/// whereas the lowest ones are located at the top.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub location: Location,
    pub value: Box<Node>,
    pub left: Option<Box<Self>>,
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
}
