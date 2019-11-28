extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Assert;

impl<E, O> VMInstruction<E, O> for Assert
where
    E: Element,
    O: ElementOperator<E>,
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
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_assert_ok() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(Assert)
            .test::<i32>(&[])
    }

    #[test]
    fn test_assert_fail() {
        let res = VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(Assert)
            .test::<i32>(&[]);

        match res {
            Err(TestingError::RuntimeError(err)) => {
                match err {
                    RuntimeError::AssertionError => {},
                    _ => assert!(false, "Expected AssertionError"),
                }
            },
            _ => assert!(false, "Expected AssertionError"),
        }
    }

    #[test]
    fn test_assert_in_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushCondition)
            .add(PushConst { value: 0.into() })
            .add(Assert)
            .add(PopCondition)
            .test::<i32>(&[])
    }
}
