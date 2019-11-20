extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Assert;

impl<E, O> VMInstruction<E, O> for Assert
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.stack_pop()?;
        vm.get_operator().assert(value)?;
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
}
