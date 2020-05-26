use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::Store;

impl<E, CS> VMInstruction<E, CS> for Store
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
