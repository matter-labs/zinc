extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Not;

impl<E, CS> VMInstruction<E, CS> for Not
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;

        let not = vm.operations().not(value)?;

        vm.push(Cell::Value(not))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_not() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(0.into()))
            .add(Not)
            .add(PushConst::new_untyped(1.into()))
            .add(Not)
            .test(&[0, 1])
    }
}
