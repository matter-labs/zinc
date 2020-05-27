use crate::core::{InternalVM, VMInstruction, VirtualMachine};
use crate::error::Result;
use crate::gadgets;
use crate::Engine;

use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::BitwiseOr;

impl<E, CS> VMInstruction<E, CS> for BitwiseOr
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::bit_or(cs.namespace(|| "bit_and"), &left, &right)?;
        vm.push(result.into())
    }
}
