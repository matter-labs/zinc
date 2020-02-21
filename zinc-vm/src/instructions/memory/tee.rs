use crate::{Result, Engine};
use crate::core::{InternalVM, VMInstruction, VirtualMachine};
use bellman::ConstraintSystem;
use zinc_bytecode::instructions::Tee;

impl<E, CS> VMInstruction<E, CS> for Tee
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
