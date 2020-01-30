use crate::gadgets::{Gadgets, ScalarType};
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::PushConst;
use franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for PushConst
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let op = vm.operations();
        let data_type = ScalarType {
            signed: self.is_signed,
            length: self.bit_length,
        };
        let value = op.constant_bigint_typed(&self.value, data_type)?;
        vm.push(Cell::Value(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};

    #[test]
    fn test_push() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(42.into()))
            .add(PushConst::new_untyped(0xABCD.into()))
            .add(PushConst::new_untyped((-1).into()))
            .add(PushConst::new_untyped((-1000).into()))
            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
