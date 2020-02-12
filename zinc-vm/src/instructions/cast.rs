extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::gadgets::IntegerType;
use crate::Engine;
use zinc_bytecode::instructions::Cast;

impl<E, CS> VMInstruction<E, CS> for Cast
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.value()?;
        let int_type = IntegerType {
            signed: self.signed,
            length: self.length,
        };

        let condition = vm.condition_top()?;
        let new_value = vm
            .operations()
            .assert_type(condition, old_value, int_type.into())?;

        vm.push(Cell::Value(new_value))
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_cast() -> Result<(), TestingError> {
        Ok(())
    }
}
