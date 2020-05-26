use crate::core::VirtualMachine;
use crate::core::{InternalVM, VMInstruction};
use crate::gadgets::Scalar;
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::ScalarType;
use zinc_bytecode::{StorageLoad, StorageStore};

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
