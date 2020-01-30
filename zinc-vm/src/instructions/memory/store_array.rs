
use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::StoreSequence;
use franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for StoreSequence
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        for i in 0..self.len {
            let value = vm.pop()?;
            vm.store(self.address + self.len - i - 1, value)?;
        }

        Ok(())
    }
}
