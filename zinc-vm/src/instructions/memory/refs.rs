use crate::gadgets::PrimitiveOperations;
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::*;

impl<E, O> VMInstruction<E, O> for Ref
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for RefGlobal
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreSequenceByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreByIndexByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for StoreSequenceByIndexByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadSequenceByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadByIndexByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, O> VMInstruction<E, O> for LoadSequenceByIndexByRef
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
