//!
//! The `Neg` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::Neg;
use zinc_build::ScalarType;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Neg {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let unchecked_neg =
            gadgets::arithmetic::neg::neg(cs.namespace(|| "unchecked_neg"), &value)?;

        match value.get_type() {
            ScalarType::Integer(mut int_type) => {
                let condition = vm.condition_top()?;
                let cs = vm.constraint_system();
                int_type.is_signed = true;
                let neg = Scalar::conditional_type_check(
                    cs.namespace(|| "neg"),
                    &condition,
                    &unchecked_neg,
                    int_type.into(),
                )?;
                vm.push(Cell::Value(neg))
            }
            scalar_type => Err(RuntimeError::TypeError {
                expected: "integer type".to_string(),
                found: scalar_type.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;
    use zinc_build::IntegerType;
    use zinc_build::Neg;
    use zinc_build::Push;

    #[test]
    fn test_neg() -> Result<(), TestingError> {
        TestRunner::new()
            .push(Push::new(127.into(), IntegerType::U8.into()))
            .push(Neg)
            .test(&[-127])
    }
}
