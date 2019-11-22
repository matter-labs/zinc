extern crate franklin_crypto;

use zrust_bytecode::instructions::Add;
use crate::vm::VMInstruction;
use crate::element::{Element, ElementOperator};
use crate::vm::{VirtualMachine, RuntimeError};

impl<E, O> VMInstruction<E, O> for Add
where E: Element, O: ElementOperator<E>
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let sum = vm.get_operator().add(left, right)?;

        vm.stack_push(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_add() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 1.into()})
            .add(Push { value: 2.into()})
            .add(Add)
            .test(&[3])
    }
}
