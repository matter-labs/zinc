use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::LoadGlobal;

impl<E, O> VMInstruction<E, O> for LoadGlobal
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.load_global(self.address)?;
        vm.push(value)
    }
}
