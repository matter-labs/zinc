//!
//! The generator expression.
//!

pub mod element;
pub mod operand;
pub mod operator;

use self::element::Element;
use self::operand::Operand;
use self::operator::Operator;

#[derive(Debug, Default, Clone)]
pub struct Expression {
    pub elements: Vec<Element>,
}

impl Expression {
    const STACK_OPERAND_INITIAL_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY),
        }
    }

    pub fn push_operand(&mut self, operand: Operand) {
        self.elements.push(Element::Operand(operand))
    }

    pub fn push_operator(&mut self, operator: Operator) {
        self.elements.push(Element::Operator(operator))
    }
}
