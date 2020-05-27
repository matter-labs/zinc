use crate::core::RuntimeError;
use crate::core::{Cell, VMInstruction, VirtualMachine};

use crate::gadgets::Scalar;
use zinc_bytecode::instructions::PushConst;

impl<VM: VirtualMachine> VMInstruction<VM> for PushConst {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = Scalar::new_constant_bigint(&self.value, self.scalar_type)?;
        vm.push(Cell::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::scalar::IntegerType;

    #[test]
    fn test_push() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(0.into()))
            .add(PushConst::new_field(42.into()))
            .add(PushConst::new_field(0xABCD.into()))
            .add(PushConst::new((-1).into(), IntegerType::I8.into()))
            .add(PushConst::new((-1000).into(), IntegerType::I16.into()))
            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
