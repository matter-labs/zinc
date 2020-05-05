use crate::core::{InternalVM, VMInstruction, VirtualMachine};


use crate::{Engine, Result, gadgets};

use franklin_crypto::bellman::ConstraintSystem;


use zinc_bytecode::instructions::BitNot;

impl<E, CS> VMInstruction<E, CS> for BitNot
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
