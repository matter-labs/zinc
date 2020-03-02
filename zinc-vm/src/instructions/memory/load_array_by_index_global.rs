use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::LoadSequenceByIndexGlobal;

impl<E, CS> VMInstruction<E, CS> for LoadSequenceByIndexGlobal
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::with_capacity(self.array_len);
        for i in 0..self.array_len {
            let value = vm.load_global(self.address + i)?.value()?;
            array.push(value);
        }

        let mut values = Vec::with_capacity(self.value_len);
        for i in 0..self.value_len {
            let offset = vm
                .operations()
                .constant_bigint(&i.into(), index.get_type())?;
            let address = vm.operations().add(index.clone(), offset)?;

            let condition = vm.condition_top()?;
            let value = vm.operations().conditional_array_get(&condition, array.as_slice(), &address)?;
            values.push(value);
        }

        for value in values.into_iter() {
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}
