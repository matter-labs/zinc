use crate::core::{VMInstruction, VirtualMachine};
use crate::{Engine, Result, RuntimeError};

use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::BitOr;
use crate::gadgets::utils::{bigint_to_fr, fr_to_bigint};
use crate::gadgets::{Scalar, ScalarType, ScalarTypeExpectation};

impl<E, CS> VMInstruction<E, CS> for BitOr
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, CS>) -> Result {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

        let scalar_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let left_value = fr_to_bigint(&left.get_constant()?, scalar_type.is_signed());
        let right_value = fr_to_bigint(&right.get_constant()?, scalar_type.is_signed());

        let result_value = &left_value | &right_value;

        let result_fr = bigint_to_fr::<E>(&result_value)
            .ok_or(RuntimeError::ValueOverflow {
                value: result_value,
                scalar_type
            })?;
        let result = Scalar::new_constant_fr(result_fr, scalar_type);
        vm.push(result.into())
    }
}
