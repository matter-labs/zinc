extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::PopCondition;

impl<E, O> VMInstruction<E, O> for PopCondition
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.condition_pop()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};
    use zrust_bytecode::{Push, PushCondition};

    #[test]
    fn test_pop_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 0.into() })
            .add(PushCondition)
            .add(PopCondition)

            .test::<i32>(&[])
    }
}
