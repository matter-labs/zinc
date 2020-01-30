
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::StoreByIndex;
use franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for StoreByIndex
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;
        let index = vm.pop()?.value()?;

        let mut array = Vec::new();
        for i in 0..self.len {
            array.push(vm.load(self.address + i)?.value()?);
        }

        let new_array = vm.operations().array_set(array.as_slice(), index, value)?;

        for (i, value) in new_array.into_iter().enumerate() {
            vm.store(self.address + i, Cell::Value(value))?;
        }

        Ok(())
    }
}
