extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::instructions::Div;

impl<E, O> VMInstruction<E, O> for Div
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let (div, _rem) = vm.operations().div_rem(left, right)?;

        vm.push(Cell::Value(div))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_div() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped((4).into()))
            .add(PushConst::new_untyped((9).into()))
            .add(Div)
            .add(PushConst::new_untyped((-4).into()))
            .add(PushConst::new_untyped((9).into()))
            .add(Div)
            .add(PushConst::new_untyped((4).into()))
            .add(PushConst::new_untyped((-9).into()))
            .add(Div)
            .add(PushConst::new_untyped((-4).into()))
            .add(PushConst::new_untyped((-9).into()))
            .add(Div)
            .test(&[3, -3, -2, 2])
    }
}
