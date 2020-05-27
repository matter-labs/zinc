extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};
use crate::{gadgets};
use zinc_bytecode::instructions::Assert;

impl<VM: VirtualMachine> VMInstruction<VM> for Assert {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;
        let c = vm.condition_top()?;
        let cs = vm.constraint_system();
        let not_c = gadgets::not(cs.namespace(|| "not"), &c)?;
        let cond_value = vm.operations().or(value, not_c)?;
        let message = match &self.message {
            Some(m) => Some(m.as_str()),
            None => None,
        };
        vm.operations().assert(cond_value, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};

    use zinc_bytecode::scalar::ScalarType;
    use zinc_bytecode::*;

    #[test]
    fn test_assert_ok() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(Assert::new(None))
            .test::<i32>(&[])
    }

    #[test]
    fn test_assert_fail() {
        let res = VMTestRunner::new()
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(Assert::new(None))
            .test::<i32>(&[]);

        match res {
            Err(TestingError::RuntimeError(RuntimeError::AssertionError(_))) => {}
            _ => panic!("Expected assertion error"),
        }
    }

    #[test]
    fn test_assert_in_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(If)
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(Assert::new(None))
            .add(EndIf)
            .test::<i32>(&[])
    }
}
