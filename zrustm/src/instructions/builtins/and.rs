extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::And;

impl<E, O> VMInstruction<E, O> for And
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?;
        let right = vm.pop()?;

        let and = vm.get_operator().and(left, right)?;

        vm.push(and)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_and() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 0.into() })
            .add(And)
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 1.into() })
            .add(And)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 0.into() })
            .add(And)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 1.into() })
            .add(And)
            .test(&[1, 0, 0, 0])
    }
}
