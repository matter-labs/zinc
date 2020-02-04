use crate::core::{VMInstruction, VirtualMachine};
use crate::{Engine, RuntimeError};
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::*;

impl<E, CS> VMInstruction<E, CS> for FileMarker
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for FunctionMarker
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}

impl<E, CS> VMInstruction<E, CS> for LineMarker
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
