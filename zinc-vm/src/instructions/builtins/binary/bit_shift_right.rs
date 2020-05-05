use crate::core::{InternalVM, VMInstruction, VirtualMachine};


use crate::{Engine, Result, gadgets};

use franklin_crypto::bellman::ConstraintSystem;



use zinc_bytecode::instructions::BitShiftRight;

impl<E, CS> VMInstruction<E, CS> for BitShiftRight
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let shift = vm.pop()?.value()?;
        let num = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::shift_right(cs.namespace(|| "shift right"), &num, &shift)?;
        vm.push(result.into())
    }
}
