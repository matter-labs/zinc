use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::LoadByIndex;

impl<E, O> VMInstruction<E, O> for LoadByIndex
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::new();
        for i in 0..self.len {
            array.push(vm.load(self.address + i)?.value()?);
        }

        let value = vm.operations().array_get(array.as_slice(), index)?;
        vm.push(Cell::Value(value))
    }
}
