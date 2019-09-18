//!
//! Auxiliary functions.
//!

use bellman::pairing::ff::BitIterator;
use bellman::ConstraintSystem;
use bellman::LinearCombination;
use bellman::SynthesisError;
use ff::Field;
use ff::PrimeField;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::num::Num;
use franklin_crypto::circuit::Assignment;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;
use pairing::Engine;

pub fn pack_bits_to_element<CS>(
    mut system: CS,
    bits: &[Boolean],
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let mut data_from_lc = Num::<Bn256>::zero();
    let mut coef = Fr::one();

    for bit in bits {
        data_from_lc = data_from_lc.add_bool_with_coeff(CS::one(), &bit, coef);
        coef.double();
    }

    let data_packed = AllocatedNum::alloc(system.namespace(|| "allocate_packed_number"), || {
        Ok(*data_from_lc.get_value().get()?)
    })?;

    system.enforce(
        || "pack_bits_to_number",
        |lc| lc + data_packed.get_variable(),
        |lc| lc + CS::one(),
        |_| data_from_lc.lc(Fr::one()),
    );

    Ok(data_packed)
}

pub fn field_into_allocated_bits_le_fixed<E, CS, F>(
    mut system: CS,
    field: Option<F>,
    bitlength: usize,
) -> Result<Vec<AllocatedBit>, SynthesisError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
    F: PrimeField,
{
    assert!(bitlength <= F::NUM_BITS as usize);

    // Deconstruct in the big-endian bit order
    let values = match field {
        Some(ref value) => {
            let mut field_char = BitIterator::new(F::char());
            let mut tmp = Vec::with_capacity(F::NUM_BITS as usize);

            let mut found_one = false;
            for bit in BitIterator::new(value.into_repr()) {
                // Skip leading bits
                found_one |= field_char.next().unwrap();
                if !found_one {
                    continue;
                }

                tmp.push(Some(bit));
            }

            assert_eq!(tmp.len(), F::NUM_BITS as usize);

            tmp
        }
        None => vec![None; F::NUM_BITS as usize],
    };

    // Allocate in the little-endian bit order
    let bits = values
        .into_iter()
        .rev()
        .enumerate()
        .take(bitlength)
        .map(|(index, bit)| AllocatedBit::alloc(system.namespace(|| format!("bit {}", index)), bit))
        .collect::<Result<Vec<_>, SynthesisError>>()?;

    Ok(bits)
}

pub fn into_bits_le_fixed<CS>(
    mut system: CS,
    number: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Vec<Boolean>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let bits = field_into_allocated_bits_le_fixed(&mut system, number.get_value(), bitlength)?;
    let mut lc = LinearCombination::zero();
    let mut coef = Fr::one();

    for bit in bits.iter() {
        lc = lc + (coef, bit.get_variable());
        coef.double();
    }

    lc = lc - number.get_variable();

    system.enforce(|| "unpacking_constraint", |lc| lc, |lc| lc, |_| lc);

    Ok(bits.into_iter().map(|b| Boolean::from(b)).collect())
}
