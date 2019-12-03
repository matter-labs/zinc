//!
//! The circuit.
//!

use crate::syntax::OuterStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct CircuitProgram {
    pub statements: Vec<OuterStatement>,
}
