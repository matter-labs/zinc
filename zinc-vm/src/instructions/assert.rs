extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Assert;

impl<E, CS> VMInstruction<E, CS> for Assert
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;
        let c = vm.condition_top()?;
        let not_c = vm.operations().not(c)?;
        let cond_value = vm.operations().or(value, not_c)?;
        vm.operations().assert(cond_value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};

    use zinc_bytecode::*;

    #[test]
    fn test_assert_ok() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(1.into(), false, 1))
            .add(Assert)
            .test::<i32>(&[])
    }

    #[test]
    #[should_panic]
    fn test_assert_fail() {
        VMTestRunner::new()
            .add(PushConst::new(0.into(), false, 1))
            .add(Assert)
            .test::<i32>(&[])
            .expect("Expected unsatisfied CS")
    }

    #[test]
    fn test_assert_in_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(0.into(), false, 1))
            .add(If)
            .add(PushConst::new(0.into(), false, 1))
            .add(Assert)
            .add(EndIf)
            .test::<i32>(&[])
    }
}
