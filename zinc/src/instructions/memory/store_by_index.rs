extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::StoreByIndex;

impl<E, O> VMInstruction<E, O> for StoreByIndex
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let index = vm.pop()?;
        let value = vm.pop()?;

        let mut array = Vec::new();
        for i in 0..self.len {
            array.push(vm.load(self.address + i)?);
        }

        let new_array = vm.get_operator().array_set(array.as_slice(), index, value)?;

        for (i, value) in new_array.into_iter().enumerate() {
            vm.store(self.address + i, value)?;
        }

        Ok(())
    }
}
