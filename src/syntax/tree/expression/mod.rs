//!
//! The expression.
//!

mod element;
mod object;
mod operand;
mod operator;

pub use self::element::Element;
pub use self::operand::Operand;
pub use self::operator::Operator;

use serde_derive::Serialize;

use crate::lexical::Token;

use self::object::Object;

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Expression(Vec<Element>);

impl Expression {
    pub fn push_operand(&mut self, (operand, token): (Operand, Token)) {
        self.0.push(Element::new(Object::Operand(operand), token));
    }

    pub fn push_operator(&mut self, (operator, token): (Operator, Token)) {
        self.0.push(Element::new(Object::Operator(operator), token));
    }

    pub fn append(&mut self, mut expression: Expression) {
        self.0.append(&mut expression.0)
    }
}
