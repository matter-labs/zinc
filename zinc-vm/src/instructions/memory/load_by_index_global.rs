use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::LoadByIndexGlobal;

impl<E, CS> VMInstruction<E, CS> for LoadByIndexGlobal
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::new();
        for i in 0..self.len {
            array.push(vm.load_global(self.address + i)?.value()?);
        }

        let condition = vm.condition_top()?;
        let value = vm.operations().conditional_array_get(&condition, array.as_slice(), &index)?;
        vm.push(Cell::Value(value))
    }
}
