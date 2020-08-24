//!
//! The `NoOperation` instruction.
//!

use zinc_build::NoOperation;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for NoOperation {
    fn execute(self, _vm: &mut VM) -> Result<(), RuntimeError> {
        Ok(())
    }
}
