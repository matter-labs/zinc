//!
//! The state modifying `unset unconstrained` instruction.
//!

use zinc_build::UnsetUnconstrained;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for UnsetUnconstrained {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.unset_unconstrained();

        Ok(())
    }
}
