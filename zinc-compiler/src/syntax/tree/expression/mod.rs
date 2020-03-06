//!
//! The expression.
//!

pub mod array;
pub mod auxiliary;
pub mod block;
pub mod builder;
pub mod conditional;
pub mod element;
pub mod r#match;
pub mod object;
pub mod operand;
pub mod operator;
pub mod structure;
pub mod tuple;

use crate::lexical::Location;

use self::element::Element as ExpressionElement;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<ExpressionElement>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<ExpressionElement>) -> Self {
        Self { location, elements }
    }
}

impl IntoIterator for Expression {
    type Item = ExpressionElement;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}
