extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};
use crate::{gadgets};
use zinc_bytecode::instructions::Lt;

impl<VM: VirtualMachine> VMInstruction<VM> for Lt {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let lt = gadgets::lt(cs.namespace(|| "lt"), &left, &right)?;

        vm.push(Cell::Value(lt))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gadgets::utils::fr_to_bigint;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use ff::Field;
    use pairing::bn256::Fr;
    use zinc_bytecode::scalar::{IntegerType, ScalarType};
    use zinc_bytecode::*;

    #[test]
    fn simple() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(PushConst::new(1.into(), IntegerType::I8.into()))
            .add(Lt)
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(Lt)
            .add(PushConst::new(1.into(), IntegerType::I8.into()))
            .add(PushConst::new(2.into(), IntegerType::I8.into()))
            .add(Lt)
            .test(&[1, 0, 0])
    }

    #[test]
    fn edge_cases() -> Result<(), TestingError> {
        let mut max_fr = Fr::zero();
        max_fr.sub_assign(&Fr::one());
        let max = fr_to_bigint(&max_fr, false);

        VMTestRunner::new()
            .add(PushConst::new(max.clone(), ScalarType::Field))
            .add(PushConst::new(0.into(), ScalarType::Field))
            .add(Lt)
            .add(PushConst::new(0.into(), ScalarType::Field))
            .add(PushConst::new(max.clone(), ScalarType::Field))
            .add(Lt)
            .add(PushConst::new(1.into(), ScalarType::Field))
            .add(PushConst::new(max.clone(), ScalarType::Field))
            .add(Lt)
            .test(&[1, 1, 0])
    }
}
