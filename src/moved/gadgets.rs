use bellman::pairing::ff::BitIterator;
use bellman::{Circuit, ConstraintSystem, LinearCombination, SynthesisError};
use ff::{Field, PrimeField};
use franklin_crypto::circuit::baby_eddsa::EddsaSignature;
use franklin_crypto::circuit::boolean;
use franklin_crypto::circuit::boolean::{AllocatedBit, Boolean};
use franklin_crypto::circuit::ecc;
use franklin_crypto::circuit::float_point::parse_with_exponent_le;
use franklin_crypto::circuit::num::{AllocatedNum, Num};
use franklin_crypto::circuit::pedersen_hash;
use franklin_crypto::circuit::polynomial_lookup::{do_the_lookup, generate_powers};
use franklin_crypto::circuit::Assignment;
use franklin_crypto::jubjub::{FixedGenerators, JubjubEngine, JubjubParams};
use pairing::bn256::{Bn256, Fr};
use pairing::Engine;

pub fn inputize_bool<E, CS>(mut cs: CS, witness: &AllocatedBit) -> Result<(), SynthesisError>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let input = cs.alloc_input(
        || "input variable",
        || {
            if witness.get_value().grab()? {
                Ok(E::Fr::one())
            } else {
                Ok(E::Fr::zero())
            }
        },
    )?;

    cs.enforce(
        || "enforce input is correct",
        |lc| lc + input,
        |lc| lc + CS::one(),
        |lc| lc + witness.get_variable(),
    );

    Ok(())
}

pub fn pack_bits_to_element<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    bits: &[boolean::Boolean],
) -> Result<AllocatedNum<Bn256>, SynthesisError> {
    let mut data_from_lc = Num::<Bn256>::zero();
    let mut coeff = Fr::one();
    for bit in bits {
        data_from_lc = data_from_lc.add_bool_with_coeff(CS::one(), &bit, coeff);
        coeff.double();
    }

    let data_packed = AllocatedNum::alloc(cs.namespace(|| "allocate packed number"), || {
        Ok(*data_from_lc.get_value().get()?)
    })?;

    cs.enforce(
        || "pack bits to number",
        |lc| lc + data_packed.get_variable(),
        |lc| lc + CS::one(),
        |_| data_from_lc.lc(Fr::one()),
    );

    Ok(data_packed)
}

pub fn field_into_allocated_bits_le_fixed<E: Engine, CS: ConstraintSystem<E>, F: PrimeField>(
    mut cs: CS,
    value: Option<F>,
    bit_length: usize,
) -> Result<Vec<AllocatedBit>, SynthesisError> {
    assert!(bit_length < F::NUM_BITS as usize);
    // Deconstruct in big-endian bit order
    let values = match value {
        Some(ref value) => {
            let mut field_char = BitIterator::new(F::char());

            let mut tmp = Vec::with_capacity(F::NUM_BITS as usize);

            let mut found_one = false;
            for b in BitIterator::new(value.into_repr()) {
                // Skip leading bits
                found_one |= field_char.next().unwrap();
                if !found_one {
                    continue;
                }

                tmp.push(Some(b));
            }

            assert_eq!(tmp.len(), F::NUM_BITS as usize);

            tmp
        }
        None => vec![None; F::NUM_BITS as usize],
    };

    // Allocate in little-endian order
    let bits = values
        .into_iter()
        .rev()
        .enumerate()
        .take(bit_length)
        .map(|(i, b)| AllocatedBit::alloc(cs.namespace(|| format!("bit {}", i)), b))
        .collect::<Result<Vec<_>, SynthesisError>>()?;

    Ok(bits)
}

pub fn into_bits_le_fixed<CS>(
    mut cs: CS,
    number: &AllocatedNum<Bn256>,
    bit_length: usize,
) -> Result<Vec<Boolean>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let bits = field_into_allocated_bits_le_fixed(&mut cs, number.get_value(), bit_length)?;

    let mut lc = LinearCombination::zero();
    let mut coeff = Fr::one();

    for bit in bits.iter() {
        lc = lc + (coeff, bit.get_variable());

        coeff.double();
    }

    lc = lc - number.get_variable();

    cs.enforce(|| "unpacking constraint", |lc| lc, |lc| lc, |_| lc);

    Ok(bits.into_iter().map(|b| Boolean::from(b)).collect())
}

//this is used to describe inputs
//#jab: input{a: type} transpiles to: (where uintX -> bit_length=x, field-> bit_length=Fr::NUM_BITS)
pub fn alloc_input<CS: ConstraintSystem<Bn256>, F: FnOnce() -> Result<Fr, SynthesisError>>(
    mut cs: CS,
    a: F,
    bit_length: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError> {
    let number = AllocatedNum::alloc(cs.namespace(|| "allocate_number"), a)?;
    number.inputize(cs.namespace(|| "inputize"))?;
    let bits = into_bits_le_fixed(cs.namespace(|| "input_bits"), &number, bit_length)?;
    Ok((number, bits))
}

//this is used for describing external witness
//#jab: witness{a: type} transpiles to: (where uintX -> bit_length=x, field-> bit_length=Fr::NUM_BITS)
pub fn alloc_witness<CS: ConstraintSystem<Bn256>, F: FnOnce() -> Result<Fr, SynthesisError>>(
    mut cs: CS,
    a: F,
    bit_length: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError> {
    let number = AllocatedNum::alloc(cs.namespace(|| "allocate_number"), a)?;
    let bits = into_bits_le_fixed(cs.namespace(|| "input_bits"), &number, bit_length)?;
    Ok((number, bits))
}

// this is used for describing in-place witness
//#jab witness unsafe {/* rust code that returns value */} syntax
//#jab: witness_generator{a: type} transpiles to: (where uintX -> bit_length=x, field-> bit_length=Fr::NUM_BITS)
// witness generator should work the way that we get code from unsafe{} and put it as a closure in the code above (F: FnOnce() -> Result<Fr, SynthesisError>)

// #jab: Types

// #jab: uintX corresponds to (AllocatedNum<Fr>, Vec<Bool>) where Vec lenth is X, field is same where X=Fr::NUM_BITS (equals 254). Those are tuples, whether number or bit representation is used depends on the operation
// bool -> Boolean
// Vector is generic over other described types
// Struct consists of types described above

// #jab: Operators

// each of operator below should have two versions, checked and unchecked
// operation transpiles to checked version according to some logic, initially in case of explicit assignment (let c = a + b), otherwise we proceed with unchecked version to optimize
// TODO: what if long expression overflows so much that becomes valid again, we should have some special syntax for ensuring this. Possible solution is to enforce overflow check if result is possible to be greater then 254 bit
// #jab: a+b transpiles into this

pub fn sum<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bit_length: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError> {
    let sum = AllocatedNum::alloc(cs.namespace(|| "name_first+name_second"), || {
        let mut sum = a.get_value().unwrap();
        sum.add_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    cs.enforce(
        || "enforce sum",
        |lc| lc + a.get_variable() + b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + sum.get_variable(),
    );
    let bits = into_bits_le_fixed(cs.namespace(|| "sum into bits"), &sum, bit_length)?;

    Ok((sum, bits))
}

pub fn sum_unchecked<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
) -> Result<AllocatedNum<Bn256>, SynthesisError> {
    let sum = AllocatedNum::alloc(cs.namespace(|| "name_first+name_second"), || {
        let mut sum = a.get_value().unwrap();
        sum.add_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    cs.enforce(
        || "enforce sum",
        |lc| lc + a.get_variable() + b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + sum.get_variable(),
    );

    Ok(sum)
}
// #jab: a-b transpiles into this
pub fn diff<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bit_length: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError> {
    let diff = AllocatedNum::alloc(cs.namespace(|| "name_first-name_second"), || {
        let mut sum = a.get_value().unwrap();
        sum.sub_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    cs.enforce(
        || "enforce diff",
        |lc| lc + a.get_variable() - b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + diff.get_variable(),
    );
    let bits = into_bits_le_fixed(cs.namespace(|| "sum into bits"), &diff, bit_length)?;

    Ok((diff, bits))
}

// #jab: a*b transpiles into this
pub fn mul<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bit_length: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError> {
    let mul = AllocatedNum::alloc(cs.namespace(|| "name_first+name_second"), || {
        let mut mul = a.get_value().unwrap();
        mul.mul_assign(&b.get_value().unwrap());
        Ok(mul)
    })?;

    cs.enforce(
        || "enforce mul",
        |lc| lc + a.get_variable(),
        |lc| lc + b.get_variable(),
        |lc| lc + mul.get_variable(),
    );
    let bits = into_bits_le_fixed(cs.namespace(|| "sum into bits"), &mul, bit_length)?;

    Ok((mul, bits))
}

// #jab: a>b transpiles into this //TODO: this is actually a>=b
pub fn greater<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bit_length: usize,
) -> Result<Boolean, SynthesisError> {
    let (diff_a_b, diff_bits) = diff(cs.namespace(|| "a-b"), a, b, bit_length)?;
    let diff_a_b_repacked = pack_bits_to_element(cs.namespace(|| "pack a-b bits"), &diff_bits)?;
    let is_a_geq_b = Boolean::from(AllocatedNum::equals(
        cs.namespace(|| "diff equal to repacked"),
        &diff_a_b,
        &diff_a_b_repacked,
    )?);
    Ok(is_a_geq_b)
}

// #jab: a<b transpiles into this //TODO: this is actually a>=b
pub fn less<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bit_length: usize,
) -> Result<Boolean, SynthesisError> {
    greater(cs, b, a, bit_length)
}

// #jab: a==b transpiles into this //TODO: this is actually a>=b
pub fn equals<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bit_length: usize,
) -> Result<Boolean, SynthesisError> {
    Ok(Boolean::from(AllocatedNum::equals(
        cs.namespace(|| "equals"),
        a,
        b,
    )?))
}
// #jab: && -> AND, || -> NAND, ^ -> NOT  applicable only over boolean (they are all in package Boolean)

// #jab: &, | -> TODO: not described

// #jab: require
// require($expr), $expr should be resolved to Boolean
pub fn require<CS: ConstraintSystem<Bn256>>(mut cs: CS, expr: &Boolean) {
    cs.enforce(
        || "expr is true",
        |_| expr.lc(CS::one(), Fr::one()),
        |lc| lc + CS::one(),
        |lc| lc + CS::one(),
    );
}

// #jab: IF
// if cond {
//     a=b
// }else{
//     a=c
// }
// transpiles to:
// NOTE: this is described as function for an example purposes, actually it should rather work like a macros
pub fn if_else_example<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    mut a: AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    c: &AllocatedNum<Bn256>,
    cond: &Boolean,
) -> Result<(), SynthesisError> {
    a = AllocatedNum::conditionally_select(cs.namespace(|| "select first if"), &a, b, cond)?;
    a = AllocatedNum::conditionally_select(
        cs.namespace(|| "select first else"),
        &a,
        c,
        &cond.not(),
    )?;
    Ok(())
}

// #jab: FOR
// for just transpiles to for{ let cs = &mut cs.namespace(|| format!("cycle_name iteration {}", i)); /*rest of the code*/}
