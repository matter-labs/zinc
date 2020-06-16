//!
//! The `Push` instruction.
//!

use zinc_bytecode::Push;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Push {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = Scalar::new_constant_bigint(self.value, self.scalar_type)?;

        vm.push(Cell::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    use zinc_bytecode::IntegerType;

    #[test]
    fn test_push() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_bytecode::Push::new_field(0.into()))
            .push(zinc_bytecode::Push::new_field(42.into()))
            .push(zinc_bytecode::Push::new_field(0xABCD.into()))
            .push(zinc_bytecode::Push::new(
                (-1).into(),
                IntegerType::I8.into(),
            ))
            .push(zinc_bytecode::Push::new(
                (-1000).into(),
                IntegerType::I16.into(),
            ))
            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
