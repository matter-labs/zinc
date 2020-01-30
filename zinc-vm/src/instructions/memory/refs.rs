use crate::core::VMInstruction;
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::*;

impl<E, CS> VMInstruction<E, CS> for Ref
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for RefGlobal
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for StoreByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for StoreSequenceByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for StoreByIndexByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for StoreSequenceByIndexByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for LoadByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for LoadSequenceByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for LoadByIndexByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for LoadSequenceByIndexByRef
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
