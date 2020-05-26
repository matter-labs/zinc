use crate::core::{InternalVM, VMInstruction, VirtualMachine};
use crate::{Engine, Result};
use bellman::ConstraintSystem;
use zinc_bytecode::instructions::Copy;

impl<E, CS> VMInstruction<E, CS> for Copy
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let value = vm.pop()?;
        vm.push(value.clone())?;
        vm.push(value)?;

        Ok(())
    }
}
