extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Eq;

impl<E, O> VMInstruction<E, O> for Eq
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let eq = vm.get_operator().eq(left, right)?;

        vm.stack_push(eq)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_eq() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 1.into() })
            .add(Push { value: 2.into() })
            .add(Eq)
            .add(Push { value: 2.into() })
            .add(Push { value: 2.into() })
            .add(Eq)
            .add(Push { value: 2.into() })
            .add(Push { value: 1.into() })
            .add(Eq)
            .test(&[0, 1, 0])
    }
}
