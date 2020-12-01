//!
//! The `Slice` instruction.
//!

use num::bigint::ToBigInt;
use num::ToPrimitive;

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::Slice;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Slice {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let offset = vm.pop()?.try_into_value()?;

        let mut array = Vec::with_capacity(self.total_size);
        for _ in 0..self.total_size {
            let value = vm.pop()?.try_into_value()?;
            array.push(value);
        }
        array.reverse();

        let offset_usize = offset
            .to_bigint()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
            .to_usize()
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
        if offset_usize + self.slice_length > self.total_size {
            return Err(Error::IndexOutOfBounds {
                lower_bound: 0,
                upper_bound: self.total_size,
                found: offset_usize + self.slice_length,
            });
        }

        for i in 0..self.slice_length {
            let condition = vm.condition_top()?;
            let namespace = format!("conditional_get_{}", i);
            let value = gadgets::array::conditional_get(
                vm.constraint_system().namespace(|| namespace),
                &condition,
                &array[i..=array.len() - self.slice_length + i],
                &offset,
            )?;
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use num::BigInt;
    use num::One;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_slice() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new_field(BigInt::one()))
            .push(zinc_types::Push::new_field(BigInt::from(2)))
            .push(zinc_types::Push::new_field(BigInt::from(3)))
            .push(zinc_types::Push::new_field(BigInt::from(4)))
            .push(zinc_types::Push::new_field(BigInt::from(5)))
            .push(zinc_types::Push::new_field(BigInt::from(6)))
            .push(zinc_types::Push::new_field(BigInt::from(2)))
            .push(zinc_types::Slice::new(2, 5))
            .test(&[5, 4, 1])
    }
}
