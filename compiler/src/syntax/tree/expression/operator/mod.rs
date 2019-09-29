//!
//! The operator expression.
//!

mod builder;
mod element;
mod object;
mod operand;
#[allow(clippy::module_inception)]
mod operator;

pub use self::builder::Builder;
pub use self::element::Element;
pub use self::object::Object;
pub use self::operand::Operand;
pub use self::operator::Operator;

use std::fmt;

use crate::lexical::Location;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<Element>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<Element>) -> Self {
        Self { location, elements }
    }
}

impl IntoIterator for Expression {
    type Item = Element;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
