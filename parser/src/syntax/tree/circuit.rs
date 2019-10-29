//!
//! The circuit.
//!

use serde_derive::Serialize;

use crate::syntax::Field;
use crate::syntax::Statement;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CircuitProgram {
    pub inputs: Vec<Field>,
    pub witnesses: Vec<Field>,
    pub outputs: Vec<Field>,
    #[serde(skip_serializing)]
    pub statements: Vec<Statement>,
}
