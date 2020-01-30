
use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Store;
use franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Store
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm.pop()?;
        vm.store(self.index, value)
    }
}
