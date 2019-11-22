extern crate franklin_crypto;

use zrust_bytecode::instructions::Push;
use crate::vm::VMInstruction;
use crate::element::{Element, ElementOperator};
use crate::vm::{VirtualMachine, RuntimeError};

impl<E, O> VMInstruction<E, O> for Push
    where E: Element, O: ElementOperator<E>
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let op = vm.get_operator();
        let value = op.constant_bigint(&self.value)?;

        vm.stack_push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_push() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 0.into() })
            .add(Push { value: 42.into() })
            .add(Push { value: 0xABCD.into() })
            .add(Push { value: (-1).into() })
            .add(Push { value: (-1000).into() })

            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
