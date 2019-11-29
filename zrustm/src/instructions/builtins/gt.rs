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
        let left = vm.memory()?.pop()?;
        let right = vm.memory()?.pop()?;

        let gt = vm.get_operator().gt(left, right)?;

        vm.memory()?.push(gt)
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
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Gt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Gt)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Gt)
            .test(&[0, 0, 1])
    }
}
