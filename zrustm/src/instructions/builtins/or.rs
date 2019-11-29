extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Or;

impl<E, O> VMInstruction<E, O> for Or
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.memory()?.pop()?;
        let right = vm.memory()?.pop()?;

        let or = vm.get_operator().or(left, right)?;

        vm.memory()?.push(or)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_or() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 0.into() })
            .add(Or)
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 1.into() })
            .add(Or)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 0.into() })
            .add(Or)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 1.into() })
            .add(Or)
            .test(&[1, 1, 1, 0])
    }
}
