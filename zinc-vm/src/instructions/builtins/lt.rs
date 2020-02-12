extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Lt;

impl<E, CS> VMInstruction<E, CS> for Lt
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let lt = vm.operations().lt(left, right)?;

        vm.push(Cell::Value(lt))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;
    use crate::gadgets::utils::fr_to_bigint;
    use pairing::bn256::Fr;
    use ff::{Field, PrimeField};
    use num_bigint::BigInt;

    #[test]
    fn simple() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(2.into(), true, 8))
            .add(PushConst::new(1.into(), true, 8))
            .add(Lt)
            .add(PushConst::new(2.into(), true, 8))
            .add(PushConst::new(2.into(), true, 8))
            .add(Lt)
            .add(PushConst::new(1.into(), true, 8))
            .add(PushConst::new(2.into(), true, 8))
            .add(Lt)
            .test(&[1, 0, 0])
    }

    #[test]
    fn edge_cases() -> Result<(), TestingError> {
        let mut max_fr = Fr::zero();
        max_fr.sub_assign(&Fr::one());
        let max = fr_to_bigint(&max_fr);

        VMTestRunner::new()
            .add(PushConst::new(max.clone(), true, 8))
            .add(PushConst::new(0.into(), true, 8))
            .add(Lt)
            .add(PushConst::new(0.into(), true, 8))
            .add(PushConst::new(max.clone(), true, 8))
            .add(Lt)
            .add(PushConst::new(1.into(), true, 8))
            .add(PushConst::new(max.clone(), true, 8))
            .add(Lt)
            .test(&[1, 1, 0])
    }
}
