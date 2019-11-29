extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Ge;

impl<E, O> VMInstruction<E, O> for Ge
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.memory()?.pop()?;
        let right = vm.memory()?.pop()?;

        let ge = vm.get_operator().ge(left, right)?;

        vm.memory()?.push(ge)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_ge() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 2.into() })
            .add(Ge)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 2.into() })
            .add(Ge)
            .add(PushConst { value: 2.into() })
            .add(PushConst { value: 1.into() })
            .add(Ge)
            .test(&[0, 1, 1])
    }
}
