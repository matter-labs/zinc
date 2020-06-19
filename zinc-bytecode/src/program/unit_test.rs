//!
//! The Zinc VM bytecode unit test program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest {
    pub instructions: Vec<Instruction>,
}

impl UnitTest {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}
