//!
//! The `BitwiseOr` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::BitwiseOr;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for BitwiseOr {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();

        let result = gadgets::bitwise::or::bit_or(cs.namespace(|| "bit_or"), &left, &right)?;

        vm.push(result.into())
    }
}
