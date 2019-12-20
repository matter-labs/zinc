use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Load;

impl<E, O> VMInstruction<E, O> for Load
    where
        E: Primitive,
        O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.load(self.index)?;
        vm.push(value)
    }
}
