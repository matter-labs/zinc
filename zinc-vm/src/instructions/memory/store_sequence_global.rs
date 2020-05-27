use crate::core::RuntimeError;
use crate::core::{VMInstruction, VirtualMachine};

use zinc_bytecode::instructions::StoreSequenceGlobal;

impl<VM: VirtualMachine> VMInstruction<VM> for StoreSequenceGlobal {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.pop()?;
            vm.store_global(self.address + i, value)?;
        }

        Ok(())
    }
}
