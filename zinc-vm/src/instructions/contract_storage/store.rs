use zinc_bytecode::StorageStore;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for StorageStore {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let address = vm.pop()?.try_into_value()?;

        let mut values = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            values.push(vm.pop()?.try_into_value()?);
        }

        vm.storage_store(&address, &values)?;

        Ok(())
    }
}
