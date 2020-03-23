extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::{Engine, gadgets};
use zinc_bytecode::instructions::Ge;

impl<E, CS> VMInstruction<E, CS> for Ge
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let ge = gadgets::ge(cs.namespace(|| "ge"), &left, &right)?;

        vm.push(Cell::Value(ge))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_ge() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(2.into()))
            .add(PushConst::new_field(1.into()))
            .add(Ge)
            .add(PushConst::new_field(2.into()))
            .add(PushConst::new_field(2.into()))
            .add(Ge)
            .add(PushConst::new_field(1.into()))
            .add(PushConst::new_field(2.into()))
            .add(Ge)
            .test(&[0, 1, 1])
    }
}
