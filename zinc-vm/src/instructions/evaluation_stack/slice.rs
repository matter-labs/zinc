use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Slice;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Slice {
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
            let namespace = format!("conditional_get_{}", i);
            let value = gadgets::array::conditional_get(
                vm.constraint_system().namespace(|| namespace),
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
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_slice() -> Result<(), TestingError> {
        TestRunner::new()
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
