extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Div;

impl<E, O> VMInstruction<E, O> for Div
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.memory()?.pop()?;
        let right = vm.memory()?.pop()?;
        let (div, _rem) = vm.get_operator().div_rem(left, right)?;

        vm.memory()?.push(div)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_div() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: (4).into() })
            .add(PushConst { value: (9).into() })
            .add(Div)
            .add(PushConst { value: (-4).into() })
            .add(PushConst { value: (9).into() })
            .add(Div)
            .add(PushConst { value: (4).into() })
            .add(PushConst { value: (-9).into() })
            .add(Div)
            .add(PushConst { value: (-4).into() })
            .add(PushConst { value: (-9).into() })
            .add(Div)
            .test(&[3, -3, -2, 2])
    }
}
