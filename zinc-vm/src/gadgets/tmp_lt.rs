//! Temporary implementation of `less than` for `AllocatedNum`

use bellman::{ConstraintSystem, SynthesisError};
use ff::{Field, PrimeField};
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::{boolean::Boolean, num::AllocatedNum, Assignment};
use pairing::Engine;

fn or<E: Engine, CS: ConstraintSystem<E>>(
    mut cs: CS,
    left: &Boolean,
    right: &Boolean,
) -> Result<Boolean, SynthesisError> {
    let boolean = Boolean::and(cs.namespace(|| "and"), &left.not(), &right.not())?.not();

    let allocated = Boolean::from(AllocatedBit::alloc(
        cs.namespace(|| "or"),
        boolean.get_value(),
    )?);

    cs.enforce(
        || "eq",
        |zero| zero + CS::one(),
        |zero| zero + &boolean.lc(CS::one(), E::Fr::one()),
        |zero| zero + &allocated.lc(CS::one(), E::Fr::one()),
    );

    Ok(allocated)
}

/// Evaluates true if a < b, false otherwise.
fn less_than_fixed<E, CS>(
    mut cs: CS,
    a: &AllocatedNum<E>,
    b: &AllocatedNum<E>,
    bit_length: usize,
) -> Result<Boolean, SynthesisError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    assert!(bit_length < E::Fr::CAPACITY as usize);

    let two_power_bit_length = {
        let mut tmp = E::Fr::one();
        for _ in 0..bit_length {
            tmp.double();
        }
        tmp.sub_assign(&E::Fr::one());
        tmp
    };

    // z = 2^bit_length - a + b
    let z = AllocatedNum::alloc(cs.namespace(|| "z"), || {
        let a = a.get_value().grab()?;
        let b = b.get_value().grab()?;

        let mut result = two_power_bit_length;
        result.sub_assign(&a);
        result.add_assign(&b);
        Ok(result)
    })?;

    cs.enforce(
        || "z = 2^bit_length - a + b",
        |lc| lc + (two_power_bit_length, CS::one()) - a.get_variable() + b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + z.get_variable(),
    );

    let z_bits = z.into_bits_le_fixed(cs.namespace(|| "z_bits"), bit_length + 1)?;

    Ok(z_bits
        .last()
        .expect("bit representation of z can't be empty")
        .clone())
}

/// Evaluates true if a < b or a <= b, false otherwise.
pub fn less_than<E, CS>(
    mut cs: CS,
    a: &AllocatedNum<E>,
    b: &AllocatedNum<E>,
) -> Result<Boolean, SynthesisError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let bits_a = a.into_bits_le_strict(cs.namespace(|| "a representation"))?;
    let bits_b = b.into_bits_le_strict(cs.namespace(|| "b representation"))?;

    let lower_bits_len: usize = E::Fr::NUM_BITS as usize / 2;
    let upper_bits_len: usize = E::Fr::NUM_BITS as usize - lower_bits_len;

    let a_lower =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "a_lower"), &bits_a[..lower_bits_len])?;
    let b_lower =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "b_lower"), &bits_b[..lower_bits_len])?;

    let a_upper =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "a_upper"), &bits_a[lower_bits_len..])?;
    let b_upper =
        AllocatedNum::pack_bits_to_element(cs.namespace(|| "b_upper"), &bits_b[lower_bits_len..])?;

    let upper_a_lt_b = less_than_fixed(
        cs.namespace(|| "upper_a_lt_b"),
        &a_upper,
        &b_upper,
        upper_bits_len,
    )?;
    let lower_a_lt_b = less_than_fixed(
        cs.namespace(|| "lower_a_lt_b"),
        &a_lower,
        &b_lower,
        lower_bits_len,
    )?;
    let upper_a_eq_b = AllocatedNum::equals(cs.namespace(|| "upper_a_eq_b"), &a_upper, &b_upper)?;

    let lower_lt_and_upper_eq =
        Boolean::and(cs.namespace(|| ""), &lower_a_lt_b, &upper_a_eq_b.into())?;

    or(cs.namespace(|| "lt"), &upper_a_lt_b, &lower_lt_and_upper_eq)
}
