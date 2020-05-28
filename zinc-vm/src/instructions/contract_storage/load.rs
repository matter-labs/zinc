use zinc_bytecode::ScalarType;
use zinc_bytecode::StorageLoad;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;

impl<VM: VirtualMachine> VMInstruction<VM> for StorageLoad {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        for _ in 0..self.size {
            vm.push(Scalar::new_constant_int(0, ScalarType::Field).into())?;
        }

        Ok(())
    }
}
