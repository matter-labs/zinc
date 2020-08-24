//!
//! The `BitwiseShiftRight` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::BitwiseShiftRight;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for BitwiseShiftRight {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let shift = vm.pop()?.try_into_value()?;
        let num = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();

        let result = gadgets::bitwise::shift_right::shift_right(
            cs.namespace(|| "bit_shift_right"),
            &num,
            &shift,
        )?;

        vm.push(result.into())
    }
}
