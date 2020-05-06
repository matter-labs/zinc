use crate::core::{InternalVM, VMInstruction};
use crate::core::{VirtualMachine};
use crate::{Engine, Result};
use zinc_bytecode::{StorageStore, StorageLoad};
use franklin_crypto::bellman::ConstraintSystem;
use crate::gadgets::Scalar;
use zinc_bytecode::scalar::ScalarType;

impl<E, CS> VMInstruction<E, CS> for StorageStore
    where
        E: Engine,
        CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        for _ in 0..self.size {
            vm.pop()?;
        }
        Ok(())
    }
}

impl<E, CS> VMInstruction<E, CS> for StorageLoad
    where
        E: Engine,
        CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        for _ in 0..self.size {
            vm.push(Scalar::new_constant_int(0, ScalarType::Field).into())?;
        }
        Ok(())
    }
}
