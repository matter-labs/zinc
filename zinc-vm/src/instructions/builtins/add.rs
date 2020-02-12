extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Add;
use crate::gadgets::ScalarType;

impl<E, CS> VMInstruction<E, CS> for Add
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let unchecked_sum = vm.operations().add(left.clone(), right.clone())?;
        let condition = vm.condition_top()?;
        let sum = vm.operations().assert_type(
            condition,
            unchecked_sum,
            ScalarType::expect_same(left.get_type(), right.get_type())?
        )?;

        vm.push(Cell::Value(sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_add() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Add)
            .test(&[3])
    }
}
