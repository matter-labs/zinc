extern crate franklin_crypto;

use crate::vm::VMInstruction;
use crate::element::{Element, ElementOperator};
use crate::vm::{VirtualMachine, RuntimeError};
use zrust_bytecode::instructions::Assert;

impl<E, O> VMInstruction<E, O> for Assert
    where E: Element, O: ElementOperator<E>
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.stack_pop()?;
        let c = vm.condition_top()?;
        let not_c = vm.get_operator().not(c)?;
        let cond_value = vm.get_operator().or(value, not_c)?;
        vm.get_operator().assert(cond_value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_assert_ok() -> Result<(), TestingError> {
        VMTestRunner::new()

            .add(Push { value: 1.into() })
            .add(Assert)

            .test::<i32>(&[])
    }

    #[test]
    fn test_assert_fail() {
        let res = VMTestRunner::new()

            .add(Push { value: 0.into() })
            .add(Assert)

            .test::<i32>(&[]);

        assert_eq!(res.unwrap_err(), TestingError::RuntimeError(RuntimeError::AssertionError));
    }

    #[test]
    fn test_assert_in_condition() -> Result<(), TestingError> {
        VMTestRunner::new()

            .add(Push { value: 0.into() })
            .add(PushCondition)
            .add(Push { value: 0.into() })
            .add(Assert)
            .add(PopCondition)

            .test::<i32>(&[])
    }
}
