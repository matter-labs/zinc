use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};


use zinc_bytecode::instructions::Store;

impl<VM: VirtualMachine> VMInstruction<VM> for Store {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?;
        vm.store(self.index, value)
    }
}
