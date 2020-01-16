use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, RuntimeError, VMInstruction, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::Slice;

impl<E, O> VMInstruction<E, O> for Slice
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let mut array = Vec::with_capacity(self.array_len);
        for _ in 0..self.array_len {
            let value = vm.pop()?.value()?;
            array.push(value);
        }
        array.reverse();

        let offset = vm.pop()?.value()?;

        for i in 0..self.slice_len {
            let index = match offset.get_data_type() {
                None => vm.operations().constant_bigint(&i.into())?,
                Some(data_type) => vm
                    .operations()
                    .constant_bigint_typed(&i.into(), data_type)?,
            };
            let address = vm.operations().add(offset.clone(), index)?;

            let value = vm.operations().array_get(array.as_slice(), address)?;
            vm.push(Cell::Value(value))?;
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
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(3.into()))
            .add(PushConst::new_untyped(4.into()))
            .add(PushConst::new_untyped(5.into()))
            .add(PushConst::new_untyped(6.into()))
            .add(Slice::new(5, 2))
            .test(&[5, 4, 1])
    }
}
