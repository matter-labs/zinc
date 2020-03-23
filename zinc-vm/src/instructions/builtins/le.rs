extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::{gadgets, Engine};
use zinc_bytecode::instructions::Le;

impl<E, CS> VMInstruction<E, CS> for Le
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let le = gadgets::le(cs.namespace(|| "le"), &left, &right)?;

        vm.push(Cell::Value(le))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::scalar::IntegerType;
    use zinc_bytecode::*;

    #[test]
    fn test_le() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(PushConst::new(1.into(), IntegerType::I8.into()))
            .add(Le)
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(Le)
            .add(PushConst::new(1.into(), IntegerType::I8.into()))
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(Le)
            .add(PushConst::new((-2).into(), IntegerType::I8.into()))
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(Le)
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(PushConst::new((-2).into(), IntegerType::I8.into()))
            .add(Le)
            .test(&[0, 1, 1, 1, 0])
    }
}
