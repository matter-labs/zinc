use franklin_crypto::bellman::ConstraintSystem;

use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::variant::Variant as ScalarVariant;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

/// This gadget only enforces 0 <= index < array.len() if condition is true
pub fn conditional_get<E, CS>(
    cs: CS,
    _condition: &Scalar<E>,
    array: &[Scalar<E>],
    index: &Scalar<E>,
) -> Result<Scalar<E>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    if !index.is_constant() {
        return Err(RuntimeError::WitnessArrayIndex);
    }
    // let zero = Scalar::new_constant_int(0, index.get_type());
    // let index = gadgets::conditional_select::conditional_select(self.cs_namespace(), condition, index, &zero)?;
    enforcing_get(cs, array, &index)
}

/// This gadget enforces 0 <= index < array.len()
pub fn enforcing_get<E, CS>(
    mut cs: CS,
    array: &[Scalar<E>],
    index: &Scalar<E>,
) -> Result<Scalar<E>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    assert!(!array.is_empty(), "reading from empty array");

    let length = Scalar::new_constant_usize(array.len(), index.get_type());
    let lt = gadgets::comparison::lesser_than(cs.namespace(|| "lt"), index, &length)?;
    gadgets::assert::assert(cs.namespace(|| "assert"), lt, Some("index out of bounds"))?;

    match index.get_variant() {
        ScalarVariant::Constant(_) => {
            let i = index.get_constant_usize()?;
            if i >= array.len() {
                return Err(RuntimeError::IndexOutOfBounds {
                    lower_bound: 0,
                    upper_bound: array.len(),
                    actual: i,
                });
            }
            Ok(array[i].clone())
        }
        _ => {
            Err(RuntimeError::WitnessArrayIndex)
            // let mut cs = self.cs_namespace();
            // let num_bits = math::log2ceil(array.len());
            // let bits_le = index.to_expression::<CS>().into_bits_le_fixed(
            //     cs.namespace(|| "into_bits"),
            //     num_bits
            // )?;
            // let bits_be = bits_le
            //     .into_iter()
            //     .rev()
            //     .enumerate()
            //     .map(|(i, bit)| {
            //         Scalar::from_boolean(cs.namespace(|| format!("bit {}", i)), bit)
            //     })
            //     .collect::<Result<Vec<Scalar<E>>, RuntimeError>>()?;

            // gadgets::recursive_select(
            //     cs.namespace(|| "recursive_select"),
            //     &bits_be,
            //     array
            // )
        }
    }
}

pub fn set<E>(
    array: &[Scalar<E>],
    index: Scalar<E>,
    value: Scalar<E>,
) -> Result<Vec<Scalar<E>>, RuntimeError>
where
    E: IEngine,
{
    let mut new_array = Vec::from(array);

    match index.get_variant() {
        ScalarVariant::Constant(_) => {
            let i = index.get_constant_usize()?;
            if i >= array.len() {
                return Err(RuntimeError::IndexOutOfBounds {
                    lower_bound: 0,
                    upper_bound: array.len(),
                    actual: i,
                });
            }
            new_array[i] = value;
        }
        _ => {
            return Err(RuntimeError::WitnessArrayIndex);
            // let mut new_array = Vec::new();

            // for (i, p) in array.iter().enumerate() {
            //     let curr_index = Scalar::new_constant_int(i, ScalarType::Field);
            //     let is_current_index = self.eq(curr_index, index.clone())?;
            //     let cs = self.cs_namespace();
            //     let value = gadgets::conditional_select::conditional_select(cs, &is_current_index, &value, p)?;
            //     new_array.push(value);
            // }
        }
    };

    Ok(new_array)
}
