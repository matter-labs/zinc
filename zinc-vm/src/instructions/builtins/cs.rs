extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::ConditionalSelect;

impl<E, O> VMInstruction<E, O> for ConditionalSelect
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let condition = vm.pop()?.value()?;
        let if_true = vm.pop()?.value()?;
        let if_false = vm.pop()?.value()?;

        let selected = vm
            .operations()
            .conditional_select(condition, if_true, if_false)?;

        vm.push(Cell::Value(selected))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::PushConst;

    #[test]
    fn test_cs() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(1337.into()))
            .add(PushConst::new_untyped(42.into()))
            .add(PushConst::new_untyped(0.into()))
            .add(ConditionalSelect)
            .add(PushConst::new_untyped(420.into()))
            .add(PushConst::new_untyped(69.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(ConditionalSelect)
            .test(&[69, 1337])
    }
}
