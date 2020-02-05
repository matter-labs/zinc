use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::Swap;

impl<E, CS> VMInstruction<E, CS> for Swap
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let a = vm.pop()?;
        let b = vm.pop()?;
        vm.push(a)?;
        vm.push(b)
    }
}
