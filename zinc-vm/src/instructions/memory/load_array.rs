use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::LoadSequence;

impl<E, CS> VMInstruction<E, CS> for LoadSequence
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.load(self.address + self.len - i - 1)?;
            vm.push(value)?;
        }

        Ok(())
    }
}
