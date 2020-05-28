use zinc_bytecode::Copy;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Copy {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?;
        vm.push(value.clone())?;
        vm.push(value)?;

        Ok(())
    }
}
