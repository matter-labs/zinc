extern crate franklin_crypto;


use crate::core::{RuntimeError, VMInstruction, VirtualMachine};

use zinc_bytecode::instructions::NoOperation;

impl<VM: VirtualMachine> VMInstruction<VM> for NoOperation {
    fn execute(&self, _vm: &mut VM) -> Result<(), RuntimeError> {
        Ok(())
    }
}
