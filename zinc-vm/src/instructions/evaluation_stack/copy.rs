//!
//! The `Copy` instruction.
//!

use zinc_types::Copy;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Copy {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let value = vm.pop()?;
        vm.push(value.clone())?;
        vm.push(value)?;

        Ok(())
    }
}
