//!
//! The syntax circuit.
//!

use crate::syntax::Inputs;
use crate::syntax::Witness;

#[derive(Debug)]
pub struct CircuitProgram {
    pub inputs: Inputs,
    pub witness: Witness,
}
