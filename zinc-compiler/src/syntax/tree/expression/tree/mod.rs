//!
//! The expression tree.
//!

pub mod builder;
pub mod node;

use crate::lexical::Location;

use self::node::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub location: Location,
    pub value: Box<Node>,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

impl Tree {
    pub fn new(location: Location, value: Node, left: Option<Self>, right: Option<Self>) -> Self {
        Self {
            location,
            value: Box::new(value),
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }
}
