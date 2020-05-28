use zinc_bytecode::NoOperation;

use crate::error::RuntimeError;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;

impl<VM: VirtualMachine> VMInstruction<VM> for NoOperation {
    fn execute(&self, _vm: &mut VM) -> Result<(), RuntimeError> {
        Ok(())
    }
}
