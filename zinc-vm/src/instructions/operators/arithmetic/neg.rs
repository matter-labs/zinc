//!
//! The `Neg` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::Neg;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Neg {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let value = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let unchecked_neg =
            gadgets::arithmetic::neg::neg(cs.namespace(|| "unchecked_neg"), &value)?;

        match value.get_type() {
            zinc_types::ScalarType::Integer(mut int_type) => {
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
            scalar_type => Err(Error::TypeError {
                expected: "integer type".to_owned(),
                found: scalar_type.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;

    use zinc_types::Neg;
    use zinc_types::Push;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_neg() -> Result<(), TestingError> {
        TestRunner::new()
            .push(Push::new(
                BigInt::from(127),
                zinc_types::IntegerType::U8.into(),
            ))
            .push(Neg)
            .test(&[-127])
    }
}
