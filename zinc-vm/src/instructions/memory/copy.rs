use crate::core::{InternalVM, VMInstruction, VirtualMachine};
use crate::error::Result;
use crate::Engine;
use bellman::ConstraintSystem;
use zinc_bytecode::Copy;

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
