//!
//! Functions used by generated circuits directly.
//!

mod utils;

use bellman::ConstraintSystem;
use bellman::SynthesisError;
use ff::Field;
use ff::PrimeField;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

///
/// Describes an allocated number.
///
/// Transpiles from variable allocations.
///
pub fn allocation<CS>(mut system: CS, number: &str) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        Ok(Fr::from_str(number).unwrap())
    })
}

///
/// Describes an input.
///
/// Transpiles from input declarations.
///
pub fn input_allocation<CS, F>(
    mut system: CS,
    input: F,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
    F: FnOnce() -> Result<Fr, SynthesisError>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), input)?;
    number.inputize(system.namespace(|| "inputize"))?;
    let bits = utils::into_bits_le_fixed(
        system.namespace(|| "into_bits_le_fixed"),
        &number,
        bitlength,
    )?;
    Ok((number, bits))
}

///
/// Describes a witness.
///
/// Transpiles from witness declarations.
///
pub fn witness_allocation<CS, F>(
    mut system: CS,
    witness: F,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
    F: FnOnce() -> Result<Fr, SynthesisError>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), witness)?;
    let bits = utils::into_bits_le_fixed(
        system.namespace(|| "into_bits_le_fixed"),
        &number,
        bitlength,
    )?;
    Ok((number, bits))
}

///
/// The OR logical function.
///
/// Transpiles from:
/// `{identifier} || {identifier}`
///
pub fn or<CS>(system: CS, a: &Boolean, b: &Boolean) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    Ok(Boolean::and(system, &a.not(), &b.not())?.not())
}

///
/// The XOR logical function.
///
/// Transpiles from:
/// `{identifier} ^^ {identifier}`
///
pub fn xor<CS>(system: CS, a: &Boolean, b: &Boolean) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    Ok(Boolean::xor(system, a, b)?)
}

///
/// The AND logical function.
///
/// Transpiles from:
/// `{identifier} && {identifier}`
///
pub fn and<CS>(system: CS, a: &Boolean, b: &Boolean) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    Ok(Boolean::and(system, a, b)?)
}

///
/// The equality comparison function.
///
/// Transpiles from:
/// `{identifier} == {identifier}`
///
pub fn equals<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    Ok(Boolean::from(AllocatedNum::equals(
        system.namespace(|| "equals"),
        a,
        b,
    )?))
}

///
/// The non-equality comparison function.
///
/// Transpiles from:
/// `{identifier} != {identifier}`
///
pub fn not_equals<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    Ok(Boolean::from(AllocatedNum::equals(
        system.namespace(|| "not_equals"),
        a,
        b,
    )?)
    .not())
}

///
/// The greater-or-equality comparison function.
///
/// Transpiles from:
/// `{identifier} >= {identifier}`
///
pub fn greater_equals<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    if let Boolean::Is(bit) = Boolean::from(AllocatedNum::equals(
        system.namespace(|| "greater_equals_equality"),
        a,
        b,
    )?) {
        if let Some(true) = bit.get_value() {
            return Ok(Boolean::from(bit));
        }
    }

    let (diff_a_b, diff_bits) = subtraction(
        system.namespace(|| "greater_equals_subtraction"),
        a,
        b,
        bitlength,
    )?;

    let diff_a_b_repacked =
        utils::pack_bits_to_element(system.namespace(|| "repacked"), &diff_bits)?;

    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| "equals_to_repacked"),
        &diff_a_b,
        &diff_a_b_repacked,
    )?);

    Ok(result)
}

///
/// The lesser-or-equality comparison function.
///
/// Transpiles from:
/// `{identifier} <= {identifier}`
///
pub fn lesser_equals<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    if let Boolean::Is(bit) = Boolean::from(AllocatedNum::equals(
        system.namespace(|| "lesser_equals_equality"),
        a,
        b,
    )?) {
        if let Some(true) = bit.get_value() {
            return Ok(Boolean::from(bit));
        }
    }

    let (diff_b_a, diff_bits) = subtraction(
        system.namespace(|| "lesser_equals_subtraction"),
        b,
        a,
        bitlength,
    )?;

    let diff_b_a_repacked =
        utils::pack_bits_to_element(system.namespace(|| "repacked"), &diff_bits)?;

    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| "equals_to_repacked"),
        &diff_b_a,
        &diff_b_a_repacked,
    )?);

    Ok(result)
}

///
/// The greater comparison function.
///
/// Transpiles from:
/// `{identifier} > {identifier}`
///
pub fn greater<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let (diff_a_b, diff_bits) =
        subtraction(system.namespace(|| "greater_subtraction"), a, b, bitlength)?;

    let diff_a_b_repacked =
        utils::pack_bits_to_element(system.namespace(|| "repacked"), &diff_bits)?;

    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| "equals_to_repacked"),
        &diff_a_b,
        &diff_a_b_repacked,
    )?);

    Ok(result)
}

///
/// The lesser comparison function.
///
/// Transpiles from:
/// `{identifier} < {identifier}`
///
pub fn lesser<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let (diff_b_a, diff_bits) =
        subtraction(system.namespace(|| "lesser_subtraction"), b, a, bitlength)?;

    let diff_b_a_repacked =
        utils::pack_bits_to_element(system.namespace(|| "repacked"), &diff_bits)?;

    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| "equals_to_repacked"),
        &diff_b_a,
        &diff_b_a_repacked,
    )?);

    Ok(result)
}

///
/// The addition function.
///
/// Transpiles from:
/// `{identifier} + {identifier}`
///
pub fn addition<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        let mut sum = a.get_value().unwrap();
        sum.add_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    system.enforce(
        || "enforce",
        |lc| lc + a.get_variable() + b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + number.get_variable(),
    );
    let bits = utils::into_bits_le_fixed(
        system.namespace(|| "into_bits_le_fixed"),
        &number,
        bitlength,
    )?;

    Ok((number, bits))
}

///
/// The subtraction function.
///
/// Transpiles from:
/// `{identifier} - {identifier}`
///
pub fn subtraction<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        let mut sum = a.get_value().unwrap();
        sum.sub_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    system.enforce(
        || "enforce",
        |lc| lc + a.get_variable() - b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + number.get_variable(),
    );
    let bits = utils::into_bits_le_fixed(
        system.namespace(|| "into_bits_le_fixed"),
        &number,
        bitlength,
    )?;

    Ok((number, bits))
}

///
/// The multiplication function.
///
/// Transpiles from:
/// `{identifier} * {identifier}`
///
pub fn multiplication<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        let mut mul = a.get_value().unwrap();
        mul.mul_assign(&b.get_value().unwrap());
        Ok(mul)
    })?;

    system.enforce(
        || "enforce",
        |lc| lc + a.get_variable(),
        |lc| lc + b.get_variable(),
        |lc| lc + number.get_variable(),
    );
    let bits = utils::into_bits_le_fixed(
        system.namespace(|| "into_bits_le_fixed"),
        &number,
        bitlength,
    )?;

    Ok((number, bits))
}

///
/// The casting function.
///
/// Transpiles from:
/// `{identifier} as {type}`
///
pub fn casting<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    AllocatedNum::alloc(system.namespace(|| "alloc"), || Ok(a.get_value().unwrap()))
}

///
/// The negation function.
///
/// Transpiles from:
/// `-{identifier}`
///
pub fn negation<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        let mut negated = Fr::zero();
        negated.sub_assign(&a.get_value().unwrap());
        Ok(negated)
    })?;

    system.enforce(
        || "enforce",
        |lc| lc + a.get_variable(),
        |lc| lc - CS::one(),
        |lc| lc + number.get_variable(),
    );
    let bits = utils::into_bits_le_fixed(
        system.namespace(|| "into_bits_le_fixed"),
        &number,
        bitlength,
    )?;

    Ok((number, bits))
}

///
/// The NOT logical function.
///
/// Transpiles from:
/// `!{identifier}`
///
pub fn not<CS>(_system: CS, a: &Boolean) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    Ok(a.not())
}

///
/// The require function.
///
/// Transpiles from:
/// `require({expression}, {string});`
///
pub fn require<CS>(mut system: CS, value: &Boolean, annotation: &str)
where
    CS: ConstraintSystem<Bn256>,
{
    system.enforce(
        || annotation,
        |_| value.lc(CS::one(), Fr::one()),
        |lc| lc + CS::one(),
        |lc| lc + CS::one(),
    );
}

///
/// The conditional expression.
///
/// Transpiles from:
/// if {expression} {
///     {statement}*
///     {expression}?
/// } else {
///     {statement}*
///     {expression}?
/// }
///
pub fn conditional<CS>(
    mut cs: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    condition: &Boolean,
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    AllocatedNum::conditionally_select(cs.namespace(|| "conditionally_select"), a, b, condition)
}
