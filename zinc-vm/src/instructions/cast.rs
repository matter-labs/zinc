extern crate franklin_crypto;

use crate::gadgets::{Gadgets, ScalarType};
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Cast;
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Cast
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.value()?;
        let data_type = ScalarType {
            signed: self.signed,
            length: self.length,
        };
        let new_value = vm.operations().set_type(old_value, data_type)?;
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
