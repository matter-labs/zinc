extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::PushCondition;

impl<E, O> VMInstruction<E, O> for PushCondition
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.pop()?;
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
    use zinc_bytecode::PushConst;

    #[test]
    fn test_push_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushCondition)
            .test::<i32>(&[])
    }
}
