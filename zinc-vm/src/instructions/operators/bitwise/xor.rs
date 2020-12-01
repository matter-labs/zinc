//!
//! The `BitwiseXor` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::BitwiseXor;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for BitwiseXor {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();

        let result = gadgets::bitwise::xor::bit_xor(cs.namespace(|| "bit_xor"), &left, &right)?;

        vm.push(result.into())
    }
}
