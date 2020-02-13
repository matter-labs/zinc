use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::PushConst;

impl<E, CS> VMInstruction<E, CS> for PushConst
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm
            .operations()
            .constant_bigint(&self.value, self.scalar_type)?;
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
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(42.into()))
            .add(PushConst::new_untyped(0xABCD.into()))
            .add(PushConst::new((-1).into(), IntegerType::I8.into()))
            .add(PushConst::new((-1000).into(), IntegerType::I16.into()))
            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
