extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Copy;

impl<E, O> VMInstruction<E, O> for Copy
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.stack_get(self.index)?;
        vm.stack_push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_copy() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 1.into()})
            .add(Push { value: 2.into()})
            .add(Push { value: 3.into()})
            .add(Copy::new(0))
            .add(Copy::new(2))
            .test(&[3, 1, 3, 2, 1])
    }
}
