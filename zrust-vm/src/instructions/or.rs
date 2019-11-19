extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Or;

impl<E, O> VMInstruction<E, O> for Or
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let or = vm.get_operator().or(left, right)?;

        vm.stack_push(or)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_or() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 0.into() })
            .add(Push { value: 0.into() })
            .add(Or)

            .add(Push { value: 0.into() })
            .add(Push { value: 1.into() })
            .add(Or)

            .add(Push { value: 1.into() })
            .add(Push { value: 0.into() })
            .add(Or)

            .add(Push { value: 1.into() })
            .add(Push { value: 1.into() })
            .add(Or)

            .test(&[1, 1, 1, 0])
    }
}
