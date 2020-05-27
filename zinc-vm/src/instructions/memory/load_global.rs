use crate::core::RuntimeError;
use crate::core::{VMInstruction, VirtualMachine};

use zinc_bytecode::LoadGlobal;

impl<VM: VirtualMachine> VMInstruction<VM> for LoadGlobal {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.load_global(self.address)?;
        vm.push(value)
    }
}
