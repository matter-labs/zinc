//!
//! The Zinc VM bytecode unit test program.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instructions::Instruction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub should_panic: bool,
    pub is_ignored: bool,
}

impl UnitTest {
    pub fn new(
        name: String,
        instructions: Vec<Instruction>,
        should_panic: bool,
        is_ignored: bool,
    ) -> Self {
        Self {
            name,
            instructions,
            should_panic,
            is_ignored,
        }
    }
}
