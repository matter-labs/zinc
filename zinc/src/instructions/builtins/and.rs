extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::And;

impl<E, O> VMInstruction<E, O> for And
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

        let and = vm.operations().and(left, right)?;

        vm.push(Cell::Value(and))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_and() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 0.into() })
            .add(And)
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 1.into() })
            .add(And)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 0.into() })
            .add(And)
            .add(PushConst { value: 1.into() })
            .add(PushConst { value: 1.into() })
            .add(And)
            .test(&[1, 0, 0, 0])
    }
}
