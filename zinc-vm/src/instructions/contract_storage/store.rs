use zinc_bytecode::StorageStore;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for StorageStore {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for _ in 0..self.size {
            vm.pop()?;
        }

        Ok(())
    }
}
