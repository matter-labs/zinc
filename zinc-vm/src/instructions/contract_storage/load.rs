use zinc_bytecode::StorageLoad;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for StorageLoad {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let address = vm.pop()?.try_into_value()?;

        let values = vm.storage_load(&address, self.size)?;

        for value in values.into_iter().rev() {
            vm.push(value.into())?;
        }

        Ok(())
    }
}
