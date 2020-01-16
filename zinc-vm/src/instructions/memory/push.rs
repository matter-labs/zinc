use crate::gadgets::{DataType, PrimitiveOperations};
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::PushConst;

impl<E, O> VMInstruction<E, O> for PushConst
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let op = vm.operations();
        let data_type = DataType {
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
