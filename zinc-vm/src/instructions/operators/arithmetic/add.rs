use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Add;
use zinc_bytecode::ScalarType;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::scalar_type::ScalarTypeExpectation;

impl<VM: VirtualMachine> VMInstruction<VM> for Add {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let sum_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_sum = gadgets::arithmetic::add::add(cs.namespace(|| "sum"), &left, &right)?;

        let sum = gadgets::types::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_sum,
            sum_type,
        )?;

        vm.push(Cell::Value(sum))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_add() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Add)
            .test(&[3])
    }
}
