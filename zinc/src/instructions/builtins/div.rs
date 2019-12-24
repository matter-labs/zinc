extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Div;

impl<E, O> VMInstruction<E, O> for Div
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;
        let (div, _rem) = vm.get_operator().div_rem(left, right)?;

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
            .add(PushConst { value: (4).into() })
            .add(PushConst { value: (9).into() })
            .add(Div)
            .add(PushConst { value: (-4).into() })
            .add(PushConst { value: (9).into() })
            .add(Div)
            .add(PushConst { value: (4).into() })
            .add(PushConst { value: (-9).into() })
            .add(Div)
            .add(PushConst { value: (-4).into() })
            .add(PushConst { value: (-9).into() })
            .add(Div)
            .test(&[3, -3, -2, 2])
    }
}
