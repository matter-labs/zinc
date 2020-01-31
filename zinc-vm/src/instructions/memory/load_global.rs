use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::LoadGlobal;

impl<E, CS> VMInstruction<E, CS> for LoadGlobal
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm.load_global(self.address)?;
        vm.push(value)
    }
}
