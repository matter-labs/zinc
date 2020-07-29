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
    /// The bytecode generation expression operand.
    Operand(Operand),
    /// The bytecode generation expression operator.
    Operator {
        /// The operator location in the source code.
        location: Location,
        /// The operator data, necessary for translating to the bytecode.
        operator: Operator,
    },
}
