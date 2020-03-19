use crate::core::{VMInstruction, VirtualMachine};
use crate::{Engine, Result};

use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::BitShiftRight;

impl<E, CS> VMInstruction<E, CS> for BitShiftRight
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result {
        unimplemented!()
    }
}
