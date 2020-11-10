use franklin_crypto::bellman::ConstraintSystem;

use crate::error::Error;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

/// This gadget only enforces 0 <= index < array.len() if condition is true
pub fn conditional_get<E, CS>(
    cs: CS,
    _condition: &Scalar<E>,
    array: &[Scalar<E>],
    index: &Scalar<E>,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    //    let zero = Scalar::new_constant_usize(0, index.get_type());
    //    let index = gadgets::select::conditional(cs.namespace(|| "index"), condition, index, &zero)?;
    enforcing_get(cs, array, &index)
}

/// This gadget enforces 0 <= index < array.len()
pub fn enforcing_get<E, CS>(
    mut cs: CS,
    array: &[Scalar<E>],
    index: &Scalar<E>,
) -> Result<Scalar<E>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    assert!(!array.is_empty(), "reading from empty array");

    let length = Scalar::new_constant_usize(array.len(), index.get_type());
    let lt = gadgets::comparison::lesser_than(cs.namespace(|| "lt"), index, &length)?;
    gadgets::require::require(cs.namespace(|| "require"), lt, Some("index out of bounds"))?;

    let i = index.to_constant_unchecked()?.get_constant_usize()?;
    if i >= array.len() {
        return Err(Error::IndexOutOfBounds {
            lower_bound: 0,
            upper_bound: array.len(),
            found: i,
        });
    }
    Ok(array[i].clone())

    //            let mut cs = cs.namespace(|| "index");
    //            let num_bits = zinc_math::log2ceil(array.len());
    //            let bits_le = index
    //                .to_expression::<CS>()
    //                .into_bits_le_fixed(cs.namespace(|| "into_bits"), num_bits)?;
    //            let bits_be = bits_le
    //                .into_iter()
    //                .rev()
    //                .enumerate()
    //                .map(|(i, bit)| Scalar::from_boolean(cs.namespace(|| format!("bit {}", i)), bit))
    //                .collect::<Result<Vec<Scalar<E>>, Error>>()?;
    //
    //            gadgets::select::recursive(cs.namespace(|| "recursive_select"), &bits_be, array)
    //        }
}

pub fn set<E, CS>(
    _cs: CS,
    array: &[Scalar<E>],
    index: Scalar<E>,
    value: Scalar<E>,
) -> Result<Vec<Scalar<E>>, Error>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    let mut new_array = Vec::from(array);

    let i = index.to_constant_unchecked()?.get_constant_usize()?;
    if i >= array.len() {
        return Err(Error::IndexOutOfBounds {
            lower_bound: 0,
            upper_bound: array.len(),
            found: i,
        });
    }
    new_array[i] = value;

    //            let mut new_array = Vec::with_capacity(array.len());
    //            for (i, p) in array.iter().enumerate() {
    //                let curr_index = Scalar::new_constant_usize(i, ScalarType::Field);
    //                let is_current_index =
    //                    gadgets::comparison::equals(cs.namespace(|| "equals"), &curr_index, &index)?;
    //                let value = gadgets::select::conditional(
    //                    cs.namespace(|| "value"),
    //                    &is_current_index,
    //                    &value,
    //                    p,
    //                )?;
    //                new_array.push(value);
    //            }
    //        }

    Ok(new_array)
}
