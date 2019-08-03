//!
//! The syntax circuit.
//!

use crate::syntax::Input;
use crate::syntax::Witness;

#[derive(Debug)]
pub struct CircuitProgram {
    pub inputs: Vec<Input>,
    pub witness: Vec<Witness>,
}
