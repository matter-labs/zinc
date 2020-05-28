use bellman::ConstraintSystem;

use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::Engine;

use zinc_utils::math;

/// Select single value from array based on index bits.
///
/// **Note**: index bits are in **big-endian**.
pub fn recursive_select<E, CS>(
    mut cs: CS,
    index_bits_be: &[Scalar<E>],
    array: &[Scalar<E>],
) -> Result<Scalar<E>, RuntimeError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    assert!(!array.is_empty(), "internal error in recursive_select 1");

    if array.len() == 1 {
        return Ok(array[0].clone());
    }

    assert!(
        !index_bits_be.is_empty(),
        "internal error in recursive_select 3"
    );

    // Skip unneeded upper bits, so we can always use the first bit for conditional select.
    let extra_bits = index_bits_be.len() - math::log2ceil(array.len());
    let index_bits_be = &index_bits_be[extra_bits..];

    let half = math::floor_to_power_of_two(array.len() - 1);
    let left = recursive_select(
        cs.namespace(|| "left recursion"),
        &index_bits_be[1..],
        &array[..half],
    )?;
    let right = recursive_select(
        cs.namespace(|| "right recursion"),
        &index_bits_be[1..],
        &array[half..],
    )?;

    gadgets::conditional_select::conditional_select(
        cs.namespace(|| "select"),
        &index_bits_be[0],
        &right,
        &left,
    )
}
