extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::ConditionalSelect;

impl<E, O> VMInstruction<E, O> for ConditionalSelect
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let condition = vm.memory()?.pop()?;
        let if_true = vm.memory()?.pop()?;
        let if_false = vm.memory()?.pop()?;

        let selected = vm
            .get_operator()
            .conditional_select(condition, if_true, if_false)?;

        vm.memory()?.push(selected)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_cs() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 1337.into() })
            .add(PushConst { value: 42.into() })
            .add(PushConst { value: 0.into() })
            .add(ConditionalSelect)
            .add(PushConst { value: 420.into() })
            .add(PushConst { value: 69.into() })
            .add(PushConst { value: 1.into() })
            .add(ConditionalSelect)
            .test(&[69, 1337])
    }
}
