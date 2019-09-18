//!
//! The expression.
//!

mod element;
mod object;
mod operand;
#[allow(clippy::module_inception)]
mod operator;

pub use self::element::Element;
pub use self::object::Object;
pub use self::operand::Operand;
pub use self::operator::Operator;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Token;

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct Expression {
    elements: Vec<Element>,
}

impl Expression {
    pub fn new(elements: Vec<Element>) -> Self {
        Self { elements }
    }

    pub fn push_operand(&mut self, (operand, token): (Operand, Token)) {
        self.elements
            .push(Element::new(Object::Operand(operand), token));
    }

    pub fn push_operator(&mut self, (operator, token): (Operator, Token)) {
        self.elements
            .push(Element::new(Object::Operator(operator), token));
    }

    pub fn append(&mut self, mut expression: Expression) {
        self.elements.append(&mut expression.elements)
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
