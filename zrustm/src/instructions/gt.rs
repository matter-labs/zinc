extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Gt;

impl<E, O> VMInstruction<E, O> for Gt
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let gt = vm.get_operator().gt(left, right)?;

        vm.stack_push(gt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_gt() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 1.into() })
            .add(Push { value: 2.into() })
            .add(Gt)
            .add(Push { value: 2.into() })
            .add(Push { value: 2.into() })
            .add(Gt)
            .add(Push { value: 2.into() })
            .add(Push { value: 1.into() })
            .add(Gt)
            .test(&[0, 0, 1])
    }
}
