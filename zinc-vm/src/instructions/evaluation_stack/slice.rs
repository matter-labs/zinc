use zinc_bytecode::Slice;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Slice {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let offset = vm.pop()?.try_into_value()?;

        let mut array = Vec::with_capacity(self.array_len);
        for _ in 0..self.array_len {
            let value = vm.pop()?.try_into_value()?;
            array.push(value);
        }
        array.reverse();

        for i in 0..self.slice_len {
            let condition = vm.condition_top()?;
            let value = vm.gadgets().conditional_array_get(
                &condition,
                &array[i..=array.len() - self.slice_len + i],
                &offset,
            )?;
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_slice() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(3.into()))
            .add(zinc_bytecode::Push::new_field(4.into()))
            .add(zinc_bytecode::Push::new_field(5.into()))
            .add(zinc_bytecode::Push::new_field(6.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Slice::new(5, 2))
            .test(&[5, 4, 1])
    }
}
