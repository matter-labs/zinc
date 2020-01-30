use crate::gadgets::Gadgets;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::LoadByIndexGlobal;
use franklin_crypto::bellman::ConstraintSystem;

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

        let value = vm.operations().array_get(array.as_slice(), index)?;
        vm.push(Cell::Value(value))
    }
}
