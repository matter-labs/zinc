extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::ConditionalSelect;

impl<E, O> VMInstruction<E, O> for ConditionalSelect
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let condition = vm.pop()?.value()?;
        let if_true = vm.pop()?.value()?;
        let if_false = vm.pop()?.value()?;

        let selected = vm
            .get_operator()
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
            .add(PushConst { value: 1337.into() })
            .add(PushConst { value: 42.into() })
            .add(PushConst { value: 0.into() })
            .add(ConditionalSelect)
            .add(PushConst { value: 420.into() })
            .add(PushConst { value: 69.into() })
            .add(PushConst { value: 1.into() })
            .add(ConditionalSelect)
            .test(&[69, 1337])
    }
}
