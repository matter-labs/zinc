extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::ConditionalSelect;

impl<E, O> VMInstruction<E, O> for ConditionalSelect
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let condition = vm.stack_pop()?;
        let if_true = vm.stack_pop()?;
        let if_false = vm.stack_pop()?;

        let selected = vm
            .get_operator()
            .conditional_select(condition, if_true, if_false)?;

        vm.stack_push(selected)
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
            .add(Push { value: 1337.into() })
            .add(Push { value: 42.into() })
            .add(Push { value: 0.into() })
            .add(ConditionalSelect)
            .add(Push { value: 420.into() })
            .add(Push { value: 69.into() })
            .add(Push { value: 1.into() })
            .add(ConditionalSelect)
            .test(&[69, 1337])
    }
}
