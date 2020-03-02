use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::gadgets;
use crate::gadgets::Scalar;
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

        let condition = vm.condition_top()?;
        let mut values = Vec::with_capacity(self.value_len);
        for i in 0..self.value_len {
            let cs = vm.constraint_system();
            let offset = Scalar::new_constant_int(i, index.get_type());
            let address = gadgets::add(cs.namespace(|| format!("add {}", i)), &index, &offset)?;
            // mem::drop(cs);
            let value =
                vm.operations()
                    .conditional_array_get(&condition, array.as_slice(), &address)?;
            values.push(value);
        }

        for value in values.into_iter() {
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}
