extern crate franklin_crypto;

use crate::vm::VMInstruction;
use crate::element::{Element, ElementOperator};
use crate::vm::{VirtualMachine, RuntimeError};
use zrust_bytecode::instructions::And;

impl<E, O> VMInstruction<E, O> for And
    where E: Element, O: ElementOperator<E>
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let and = vm.get_operator().and(left, right)?;

        vm.stack_push(and)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_and() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 0.into() })
            .add(Push { value: 0.into() })
            .add(And)

            .add(Push { value: 0.into() })
            .add(Push { value: 1.into() })
            .add(And)

            .add(Push { value: 1.into() })
            .add(Push { value: 0.into() })
            .add(And)

            .add(Push { value: 1.into() })
            .add(Push { value: 1.into() })
            .add(And)

            .test(&[1, 0, 0, 0])
    }
}
