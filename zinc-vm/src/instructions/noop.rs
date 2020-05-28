use zinc_bytecode::NoOperation;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for NoOperation {
    fn execute(&self, _vm: &mut VM) -> Result<(), RuntimeError> {
        Ok(())
    }
}
