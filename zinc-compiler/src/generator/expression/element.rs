//!
//! The generator expression element.
//!

use crate::generator::expression::operand::Operand;
use crate::generator::expression::operator::Operator;

#[derive(Debug, Clone)]
pub enum Element {
    Operand(Operand),
    Operator(Operator),
}
