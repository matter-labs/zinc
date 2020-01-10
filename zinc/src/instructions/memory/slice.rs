use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Slice;

impl<E, O> VMInstruction<E, O> for Slice
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let mut slice = Vec::with_capacity(self.slice_len);

        let offset = self
            .len
            .checked_sub(self.slice_len + self.slice_offset)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::PushConst;

    #[test]
    fn test_slice() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(3.into()))
            .add(PushConst::new_untyped(4.into()))
            .add(PushConst::new_untyped(5.into()))
            .add(PushConst::new_untyped(6.into()))
            .add(Slice::new(5, 2, 1))
            .test(&[4, 3, 1])
    }
}
