//!
//! The expression.
//!

mod element;
mod object;
mod operand;
mod operator;

pub use self::element::Element;
pub use self::object::Object;
pub use self::operand::Operand;
pub use self::operator::Operator;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::lexical::Token;

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Expression {
    pub elements: Vec<Element>,
}

impl Expression {
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

    pub fn location(&self) -> Location {
        self.elements[0].token.location
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
                .join(" ")
        )
    }
}
