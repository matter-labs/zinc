
use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Load;
use franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Load
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm.load(self.address)?;
        vm.push(value)
    }
}
