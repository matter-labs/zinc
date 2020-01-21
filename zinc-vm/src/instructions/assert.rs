extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::instructions::Assert;

impl<E, O> VMInstruction<E, O> for Assert
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;
        let c = vm.condition_top()?;
        let not_c = vm.operations().not(c)?;
        let cond_value = vm.operations().or(value, not_c)?;
        vm.operations().assert(cond_value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};

    use zinc_bytecode::*;

    #[test]
    fn test_assert_ok() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(1.into(), false, 1))
            .add(Assert)
            .test::<i32>(&[])
    }

    #[test]
    fn test_assert_fail() {
        let res = VMTestRunner::new()
            .add(PushConst::new(0.into(), false, 1))
            .add(Assert)
            .test::<i32>(&[]);

        match res {
            Err(TestingError::Unsatisfied) => {}
            _ => panic!("Expected unsatisfied CS"),
        }
    }

    #[test]
    fn test_assert_in_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(0.into(), false, 1))
            .add(If)
            .add(PushConst::new(0.into(), false, 1))
            .add(Assert)
            .add(EndIf)
            .test::<i32>(&[])
    }
}
