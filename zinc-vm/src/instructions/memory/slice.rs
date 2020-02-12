use crate::core::{Cell, InternalVM, RuntimeError, VMInstruction, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::Slice;

impl<E, CS> VMInstruction<E, CS> for Slice
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let offset = vm.pop()?.value()?;

        let mut array = Vec::with_capacity(self.array_len);
        for _ in 0..self.array_len {
            let value = vm.pop()?.value()?;
            array.push(value);
        }
        array.reverse();

        for i in 0..self.slice_len {
            let index = vm
                .operations()
                .constant_bigint(&i.into(), offset.get_type())?;
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
            .add(PushConst::new_untyped(3.into()))
            .add(PushConst::new_untyped(4.into()))
            .add(PushConst::new_untyped(5.into()))
            .add(PushConst::new_untyped(6.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Slice::new(5, 2))
            .test(&[5, 4, 1])
    }
}
