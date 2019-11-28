extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::PushCondition;

impl<E, O> VMInstruction<E, O> for PushCondition
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.frame()?.pop()?;
        let prev = vm.condition_top()?;
        let and = vm.get_operator().and(value, prev)?;
        vm.condition_push(and)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::PushConst;

    #[test]
    fn test_push_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushCondition)
            .test::<i32>(&[])
    }
}
