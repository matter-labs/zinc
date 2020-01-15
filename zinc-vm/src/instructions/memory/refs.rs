use crate::gadgets::{PrimitiveOperations};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::*;

impl<E, O> VMInstruction<E, O> for Ref
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for RefGlobal
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreSequenceByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreByIndexByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreSequenceByIndexByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadSequenceByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadByIndexByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadSequenceByIndexByRef
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
