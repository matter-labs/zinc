//!
//! The state modifying `set unconstrained` instruction.
//!

use zinc_bytecode::SetUnconstrained;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for SetUnconstrained {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.set_unconstrained();

        Ok(())
    }
}
