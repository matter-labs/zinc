extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Mul;

impl<E, O> VMInstruction<E, O> for Mul
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;
        let prod = vm.get_operator().mul(left, right)?;

        vm.stack_push(prod)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_mul() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 3.into() })
            .add(Push { value: 4.into() })
            .add(Mul)
            .test(&[12])
    }
}
