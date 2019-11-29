extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Xor;

impl<E, O> VMInstruction<E, O> for Xor
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.memory()?.pop()?;
        let right = vm.memory()?.pop()?;

        let xor = vm.get_operator().xor(left, right)?;

        vm.memory()?.push(xor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_xor() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 0.into() })
            .add(Xor)
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 1.into() })
            .add(Xor)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 0.into() })
            .add(Xor)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 1.into() })
            .add(Xor)
            .test(&[0, 1, 1, 0])
    }
}
