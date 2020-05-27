extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};

use crate::{gadgets, Engine};
use zinc_bytecode::Cast;

impl<E, CS> VMInstruction<E, CS> for Cast
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.value()?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();
        let new_value = gadgets::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &old_value,
            self.r#type.to_owned(),
        )?;

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
