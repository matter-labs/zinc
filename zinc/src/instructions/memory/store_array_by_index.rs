use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::StoreSequenceByIndex;

impl<E, O> VMInstruction<E, O> for StoreSequenceByIndex
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::with_capacity(self.array_len);
        for i in 0..self.array_len {
            let value = vm.load(self.address + i)?.value()?;
            array.push(value);
        }

        let mut values = Vec::with_capacity(self.value_len);
        for _ in 0..self.value_len {
            let value = vm.pop()?.value()?;
            values.push(value);
        }
        values.reverse();

        for (i, value) in values.into_iter().enumerate() {
            let offset = vm.operations().constant_bigint(&i.into())?;
            let address = vm.operations().add(index.clone(), offset)?;
            array = vm
                .operations()
                .array_set(array.as_slice(), address, value)?;
        }

        for (i, value) in array.into_iter().enumerate() {
            vm.store(self.address + i, Cell::Value(value))?;
        }

        Ok(())
    }
}
