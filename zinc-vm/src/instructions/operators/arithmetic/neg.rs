use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Neg;
use zinc_bytecode::ScalarType;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;

impl<VM: VirtualMachine> VMInstruction<VM> for Neg {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let unchecked_neg =
            gadgets::arithmetic::neg::neg(cs.namespace(|| "unchecked_neg"), &value)?;

        match value.get_type() {
            ScalarType::Integer(mut int_type) => {
                let condition = vm.condition_top()?;
                let cs = vm.constraint_system();
                int_type.is_signed = true;
                let neg = gadgets::types::conditional_type_check(
                    cs.namespace(|| "neg"),
                    &condition,
                    &unchecked_neg,
                    int_type.into(),
                )?;
                vm.push(Cell::Value(neg))
            }
            scalar_type => Err(RuntimeError::TypeError {
                expected: "integer type".to_string(),
                actual: scalar_type.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;
    use zinc_bytecode::IntegerType;
    use zinc_bytecode::Neg;
    use zinc_bytecode::Push;

    #[test]
    fn test_neg() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push::new(128.into(), IntegerType::U8.into()))
            .add(Neg)
            .test(&[-128])
    }
}
