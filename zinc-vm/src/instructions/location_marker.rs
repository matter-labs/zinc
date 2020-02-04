use crate::core::{VMInstruction, VirtualMachine};
use crate::{Engine, RuntimeError};
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::LocationMarker;

impl<E, CS> VMInstruction<E, CS> for LocationMarker
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        unimplemented!()
    }
}
