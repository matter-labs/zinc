//!
//! The syntax circuit.
//!

use serde_derive::Serialize;

use crate::syntax::Input;
use crate::syntax::Witness;

#[derive(Debug, Serialize)]
pub struct CircuitProgram {
    pub inputs: Vec<Input>,
    pub witnesses: Vec<Witness>,
}
