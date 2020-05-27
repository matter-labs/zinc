use crate::core::{VMInstruction, VirtualMachine};
use crate::Result;

use zinc_bytecode::{StorageLoad, StorageStore};

impl<VM: VirtualMachine> VMInstruction<VM> for StorageStore {
    fn execute(&self, vm: &mut VM) -> Result {
        let address = vm.pop()?.value()?;

        let mut values = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            let v = vm.pop()?.value()?;
            values.push(v);
        }
        values.reverse();

        vm.storage_store(&address, &values)?;

        Ok(())
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for StorageLoad {
    fn execute(&self, vm: &mut VM) -> Result {
        let address = vm.pop()?.value()?;

        let values = vm.storage_load(&address, self.size)?;

        for value in values {
            vm.push(value.into())?;
        }

        Ok(())
    }
}
