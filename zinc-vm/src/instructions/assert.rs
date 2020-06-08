use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Assert;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Assert {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.try_into_value()?;
        let c = vm.condition_top()?;
        let cs = vm.constraint_system();
        let not_c = gadgets::logical::not::not(cs.namespace(|| "not"), &c)?;
        let cond_value = gadgets::logical::or::or(cs.namespace(|| "or"), &value, &not_c)?;
        let message = match &self.message {
            Some(m) => Some(m.as_str()),
            None => None,
        };
        gadgets::assert::assert(cs, cond_value, message)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::RuntimeError;
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    use zinc_bytecode::ScalarType;

    #[test]
    fn test_assert_ok() -> Result<(), TestingError> {
        TestRunner::new()
            .add(zinc_bytecode::Push::new(1.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Assert::new(None))
            .test::<i32>(&[])
    }

    #[test]
    fn test_assert_fail() {
        let res = TestRunner::new()
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
        TestRunner::new()
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::If)
            .add(zinc_bytecode::Push::new(0.into(), ScalarType::Boolean))
            .add(zinc_bytecode::Assert::new(None))
            .add(zinc_bytecode::EndIf)
            .test::<i32>(&[])
    }
}
