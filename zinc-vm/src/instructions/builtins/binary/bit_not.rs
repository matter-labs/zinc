use crate::core::{VMInstruction, VirtualMachine, InternalVM};
use crate::{Engine, Result, RuntimeError};

use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::BitNot;
use crate::gadgets::{Scalar};
use crate::gadgets::utils::{fr_to_bigint, bigint_to_fr};

impl<E, CS> VMInstruction<E, CS> for BitNot
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result {
        let scalar = vm.pop()?.value()?;

        let scalar_type = scalar.get_type();
        let value = fr_to_bigint(&scalar.get_constant()?, scalar_type.is_signed());

        let result_value = !value;

        let result_fr = bigint_to_fr::<E>(&result_value)
            .ok_or(RuntimeError::ValueOverflow {
                value: result_value,
                scalar_type
            })?;
        let result = Scalar::new_constant_fr(result_fr, scalar_type);
        vm.push(result.into())
    }
}
