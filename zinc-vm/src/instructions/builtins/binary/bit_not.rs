use crate::core::{InternalVM, VMInstruction, VirtualMachine};

use crate::error::Result;
use crate::gadgets;
use crate::Engine;

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseNot;

impl<E, CS> VMInstruction<E, CS> for BitwiseNot
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let scalar = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::bit_not(cs.namespace(|| "bit_and"), &scalar)?;
        vm.push(result.into())
    }
}
