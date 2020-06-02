use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;
use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Cast;

impl<VM: IVirtualMachine> IExecutable<VM> for Cast {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.try_into_value()?;

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
