extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Neg;
use zinc_bytecode::scalar::ScalarType;

impl<E, CS> VMInstruction<E, CS> for Neg
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;

        let unchecked_neg = vm.operations().neg(value.clone())?;

        match value.get_type() {
            scalar_type @ ScalarType::Field | scalar_type @ ScalarType::Boolean => {
                Err(RuntimeError::TypeError {
                    expected: "integer type".to_string(),
                    actual: scalar_type.to_string(),
                })
            }
            ScalarType::Integer(mut int_type) => {
                let condition = vm.condition_top()?;
                int_type.signed = true;
                let neg = vm
                    .operations()
                    .assert_type(condition, unchecked_neg, int_type.into())?;
                vm.push(Cell::Value(neg))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_neg() -> Result<(), TestingError> {
        Ok(())
    }
}
