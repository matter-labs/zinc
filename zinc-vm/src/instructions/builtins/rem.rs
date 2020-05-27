extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::RuntimeError;
use crate::core::{Cell, VMInstruction, VirtualMachine};
use crate::gadgets;
use crate::gadgets::{ScalarType, ScalarTypeExpectation};

use zinc_bytecode::instructions::Rem;

impl<VM: VirtualMachine> VMInstruction<VM> for Rem {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let (_div, unchecked_rem) =
            gadgets::div_rem_conditional(cs.namespace(|| "div_rem"), &condition, &left, &right)?;

        let rem = gadgets::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_rem,
            ScalarType::expect_same(left.get_type(), right.get_type())?,
        )?;

        vm.push(Cell::Value(rem))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::scalar::IntegerType;
    use zinc_bytecode::*;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst::new(9.into(), IntegerType::I8.into()))
            .add(PushConst::new(4.into(), IntegerType::I8.into()))
            .add(Rem)
            .add(PushConst::new(9.into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Rem)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new(4.into(), IntegerType::I8.into()))
            .add(Rem)
            .add(PushConst::new((-9).into(), IntegerType::I8.into()))
            .add(PushConst::new((-4).into(), IntegerType::I8.into()))
            .add(Rem)
            .test(&[3, 3, 1, 1])
    }
}
