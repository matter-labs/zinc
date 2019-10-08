//!
//! The circuit.
//!

use serde_derive::Serialize;

use crate::syntax::Input;
use crate::syntax::Statement;
use crate::syntax::Witness;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CircuitProgram {
    pub inputs: Vec<Input>,
    pub witnesses: Vec<Witness>,
    #[serde(skip_serializing)]
    pub statements: Vec<Statement>,
}
