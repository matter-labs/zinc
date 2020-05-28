use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Cast;

impl<VM: VirtualMachine> VMInstruction<VM> for Cast {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.value()?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();
        let new_value = gadgets::types::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &old_value,
            self.r#type.to_owned(),
        )?;

        vm.push(Cell::Value(new_value))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;

    #[test]
    fn test_cast() -> Result<(), TestingError> {
        Ok(())
    }
}
