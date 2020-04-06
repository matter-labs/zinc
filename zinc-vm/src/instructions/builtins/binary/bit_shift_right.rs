use crate::core::{InternalVM, VMInstruction, VirtualMachine};
use crate::gadgets::utils::{bigint_to_fr, fr_to_bigint};
use crate::gadgets::{Scalar, ScalarTypeExpectation};
use crate::{Engine, Result, RuntimeError};

use franklin_crypto::bellman::ConstraintSystem;
use num_bigint::BigInt;
use num_bigint::Sign;
use num_traits::ToPrimitive;
use zinc_bytecode::instructions::BitShiftRight;

impl<E, CS> VMInstruction<E, CS> for BitShiftRight
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let scalar_type = left.get_type();

        let mut left_value = fr_to_bigint(&left.get_constant()?, scalar_type.is_signed());
        let right_value = fr_to_bigint(&right.get_constant()?, false);

        let mut mask = vec![0xFF; scalar_type.bit_length::<E>() / 8];
        if scalar_type.is_signed() {
            mask[0] = 0x7F;
        }

        left_value &= &BigInt::from_bytes_le(Sign::Plus, mask.as_slice());
        let result_value = &left_value >> right_value.to_usize().unwrap();

        let result_fr = bigint_to_fr::<E>(&result_value).ok_or(RuntimeError::ValueOverflow {
            value: result_value,
            scalar_type,
        })?;
        let result = Scalar::new_constant_fr(result_fr, scalar_type);
        vm.push(result.into())
    }
}
