use crate::core::RuntimeError;
use crate::core::{VMInstruction, VirtualMachine};

use zinc_bytecode::instructions::Swap;

impl<VM: VirtualMachine> VMInstruction<VM> for Swap {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let a = vm.pop()?;
        let b = vm.pop()?;
        vm.push(a)?;
        vm.push(b)
    }
}
