use crate::core::{VirtualMachine, VMInstruction};
use crate::{Result};

use zinc_bytecode::instructions::Tee;

impl<VM: VirtualMachine> VMInstruction<VM> for Tee {
    fn execute(&self, vm: &mut VM) -> Result {
        let value = vm.pop()?;
        vm.push(value.clone())?;
        vm.push(value)?;

        Ok(())
    }
}
