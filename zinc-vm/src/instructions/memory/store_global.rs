use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};


use zinc_bytecode::instructions::StoreGlobal;

impl<VM: VirtualMachine> VMInstruction<VM> for StoreGlobal {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?;
        vm.store_global(self.address, value)
    }
}
