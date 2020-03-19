//!
//! The generator expression operand.
//!

pub mod constant;
pub mod variable;

use self::constant::Constant;
use self::variable::Variable;

#[derive(Debug, Clone)]
pub enum Operand {
    Constant(Constant),
    Variable(Variable),
}
