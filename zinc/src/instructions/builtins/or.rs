extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Or;

impl<E, O> VMInstruction<E, O> for Or
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

        let or = vm.get_operator().or(left, right)?;

        vm.push(Cell::Value(or))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_or() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 0.into() })
            .add(Or)
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 1.into() })
            .add(Or)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 0.into() })
            .add(Or)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 1.into() })
            .add(Or)
            .test(&[1, 1, 1, 0])
    }
}
