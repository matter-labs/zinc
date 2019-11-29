extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::PopCondition;

impl<E, O> VMInstruction<E, O> for PopCondition
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.condition_pop()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::{PushConst, PushCondition};

    #[test]
    fn test_pop_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushCondition)
            .add(PopCondition)
            .test::<i32>(&[])
    }
}
