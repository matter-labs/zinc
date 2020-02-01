use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::LoadSequenceByIndex;

impl<E, CS> VMInstruction<E, CS> for LoadSequenceByIndex
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::with_capacity(self.array_len);
        for i in 0..self.array_len {
            let value = vm.load(self.address + i)?.value()?;
            array.push(value);
        }

        let mut values = Vec::with_capacity(self.value_len);
        for i in 0..self.value_len {
            let offset = match index.get_data_type() {
                None => vm.operations().constant_bigint(&i.into()),
                Some(t) => vm.operations().constant_bigint_typed(&i.into(), t),
            }?;
            let address = vm.operations().add(index.clone(), offset)?;

            let value = vm.operations().array_get(array.as_slice(), address)?;
            values.push(value);
        }

        for value in values.into_iter().rev() {
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}
