use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::*;

impl<E, O> VMInstruction<E, O> for Ref
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for RefGlobal
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreSequenceByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreByIndexByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreSequenceByIndexByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadSequenceByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadByIndexByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadSequenceByIndexByRef
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
