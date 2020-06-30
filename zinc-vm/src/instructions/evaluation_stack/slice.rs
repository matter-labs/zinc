//!
//! The `Slice` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Slice;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Slice {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let offset = vm.pop()?.try_into_value()?;

        let mut array = Vec::with_capacity(self.total_size);
        for _ in 0..self.total_size {
            let value = vm.pop()?.try_into_value()?;
            array.push(value);
        }
        array.reverse();

        // TODO: check the array bounds

        for i in 0..self.slice_size {
            let condition = vm.condition_top()?;
            let namespace = format!("conditional_get_{}", i);
            let value = gadgets::array::conditional_get(
                vm.constraint_system().namespace(|| namespace),
                &condition,
                &array[i..=array.len() - self.slice_size + i],
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
            .push(zinc_bytecode::Push::new_field(1.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Push::new_field(3.into()))
            .push(zinc_bytecode::Push::new_field(4.into()))
            .push(zinc_bytecode::Push::new_field(5.into()))
            .push(zinc_bytecode::Push::new_field(6.into()))
            .push(zinc_bytecode::Push::new_field(2.into()))
            .push(zinc_bytecode::Slice::new(2, 5))
            .test(&[5, 4, 1])
    }
}
