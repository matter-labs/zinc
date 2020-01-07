use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::LoadSequenceGlobal;

impl<E, O> VMInstruction<E, O> for LoadSequenceGlobal
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
