//!
//! The `Require` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::Require;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Require {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let value = vm.pop()?.try_into_value()?;
        let condition = vm.condition_top()?;

        let cs = vm.constraint_system();

        let not_c = gadgets::logical::not::not(cs.namespace(|| "not"), &condition)?;
        let condition = gadgets::logical::or::or(cs.namespace(|| "or"), &value, &not_c)?;

        let message = match &self.message {
            Some(message) => Some(message.as_str()),
            None => None,
        };
        gadgets::require::require(cs, condition, message)
    }
}

#[cfg(test)]
mod tests {
    use num::BigInt;
    use num::One;
    use num::Zero;

    use crate::error::Error;
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_require_ok() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new(
                BigInt::one(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Require::new(None))
            .test::<i32>(&[])
    }

    #[test]
    fn test_require_fail() {
        let res = TestRunner::new()
            .push(zinc_types::Push::new(
                BigInt::zero(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Require::new(None))
            .test::<i32>(&[]);

        match res {
            Err(TestingError::Error(Error::RequireError(_))) => {}
            _ => panic!("Expected require error"),
        }
    }

    #[test]
    fn test_require_in_condition() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new(
                BigInt::zero(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::If)
            .push(zinc_types::Push::new(
                BigInt::zero(),
                zinc_types::ScalarType::Boolean,
            ))
            .push(zinc_types::Require::new(None))
            .push(zinc_types::EndIf)
            .test::<i32>(&[])
    }
}
