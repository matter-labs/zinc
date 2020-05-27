use crate::core::{InternalVM, VMInstruction, VirtualMachine};

use crate::error::Result;
use crate::gadgets;
use crate::Engine;

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseShiftLeft;

impl<E, CS> VMInstruction<E, CS> for BitwiseShiftLeft
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let shift = vm.pop()?.value()?;
        let num = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::shift_left(cs.namespace(|| "shift left"), &num, &shift)?;
        vm.push(result.into())
    }
}
