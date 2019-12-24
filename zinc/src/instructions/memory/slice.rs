use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Slice;

impl<E, O> VMInstruction<E, O> for Slice
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let mut slice = Vec::with_capacity(self.slice_len);

        let offset = self.len.checked_sub(self.slice_len + self.slice_offset)
            .ok_or(RuntimeError::IndexOutOfBounds)?;

        for _ in 0..offset {
            vm.pop()?;
        }

        for _ in 0..self.slice_len {
            let value = vm.pop()?;
            slice.push(value);
        }

        for _ in 0..self.slice_offset {
            vm.pop()?;
        }

        for value in slice.into_iter().rev() {
            vm.push(value)?;
        }

        Ok(())
    }
}
