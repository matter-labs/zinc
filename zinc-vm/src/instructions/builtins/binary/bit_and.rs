use crate::core::{InternalVM, VMInstruction, VirtualMachine};
use crate::{Engine, Result};

use crate::gadgets;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::BitAnd;

impl<E, CS> VMInstruction<E, CS> for BitAnd
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::bit_and(cs.namespace(|| "bit_and"), &left, &right)?;
        vm.push(result.into())
    }
}
