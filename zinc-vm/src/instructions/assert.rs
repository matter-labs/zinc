use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Assert;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Assert {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.try_into_value()?;
        let c = vm.condition_top()?;
        let cs = vm.constraint_system();
        let not_c = gadgets::logical::not::not(cs.namespace(|| "not"), &c)?;
        let cond_value = vm.gadgets().or(value, not_c)?;
        let message = match &self.message {
            Some(m) => Some(m.as_str()),
            None => None,
        };
        vm.gadgets().assert(cond_value, message)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::RuntimeError;
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    use zinc_bytecode::ScalarType;

    #[test]
    fn test_assert_ok() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Assert::new(None))
            .test::<i32>(&[])
    }

    #[test]
    fn test_assert_fail() {
        let res = VMTestRunner::new()
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Assert::new(None))
            .test::<i32>(&[]);

        match res {
            Err(TestingError::RuntimeError(RuntimeError::AssertionError(_))) => {}
            _ => panic!("Expected assertion error"),
        }
    }

    #[test]
    fn test_assert_in_condition() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::If)
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Assert::new(None))
            .add(zinc_bytecode::EndIf)
            .test::<i32>(&[])
    }
}
