extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::PushCondition;

impl<E, O> VMInstruction<E, O> for PushCondition
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.stack_pop()?;
        let prev = vm.condition_top()?;
        let and = vm.get_operator().and(value, prev)?;
        vm.condition_push(and)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};
    use zrust_bytecode::Push;

    #[test]
    fn test_push_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 0.into() })
            .add(PushCondition)

            .test::<i32>(&[])
    }
}
