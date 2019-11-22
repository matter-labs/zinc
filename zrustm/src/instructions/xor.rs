extern crate franklin_crypto;

use crate::vm::VMInstruction;
use crate::element::{Element, ElementOperator};
use crate::vm::{VirtualMachine, RuntimeError};
use zrust_bytecode::instructions::Xor;

impl<E, O> VMInstruction<E, O> for Xor
    where E: Element, O: ElementOperator<E>
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let xor = vm.get_operator().xor(left, right)?;

        vm.stack_push(xor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_xor() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 0.into() })
            .add(Push { value: 0.into() })
            .add(Xor)

            .add(Push { value: 0.into() })
            .add(Push { value: 1.into() })
            .add(Xor)

            .add(Push { value: 1.into() })
            .add(Push { value: 0.into() })
            .add(Xor)

            .add(Push { value: 1.into() })
            .add(Push { value: 1.into() })
            .add(Xor)

            .test(&[0, 1, 1, 0])
    }
}
