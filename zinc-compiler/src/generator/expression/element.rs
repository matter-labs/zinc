//!
//! The generator expression element.
//!

use crate::generator::expression::operand::Operand;
use crate::generator::expression::operator::Operator;
use crate::lexical::token::location::Location;

///
/// The bytecode generator expression element.
///
#[derive(Debug, Clone)]
pub enum Element {
    Operand(Operand),
    Operator {
        location: Location,
        operator: Operator,
    },
}
