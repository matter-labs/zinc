use crate::core::RuntimeError;
use crate::core::{VMInstruction, VirtualMachine};

use zinc_bytecode::instructions::Load;

impl<VM: VirtualMachine> VMInstruction<VM> for Load {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.load(self.address)?;
        vm.push(value)
    }
}
