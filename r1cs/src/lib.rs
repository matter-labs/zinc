//!
//! R1CS functions used by generated circuits directly.
//!

mod utils;

pub use bellman::Circuit;
pub use bellman::ConstraintSystem;
pub use bellman::SynthesisError;
pub use franklin_crypto::circuit::boolean::Boolean;
pub use franklin_crypto::circuit::num::AllocatedNum;
pub use pairing::bn256::Bn256;
pub use pairing::bn256::Fr;
pub use sapling_crypto::circuit::test::TestConstraintSystem;

use ff::Field;
use ff::PrimeField;
use franklin_crypto::circuit::boolean::AllocatedBit;

///
/// Allocates a boolean.
///
/// Transpiles from variable allocations.
///
pub fn allocate_boolean<S>(mut system: S, value: bool) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::from(AllocatedBit::alloc(
        system.namespace(|| "boolean_alloc"),
        Some(value),
    )?))
}

///
/// Allocates an allocated number.
///
/// Transpiles from variable allocations.
///
pub fn allocate_number<S>(mut system: S, value: &str) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    AllocatedNum::alloc(system.namespace(|| "number_alloc"), || {
        Ok(Fr::from_str(value).unwrap())
    })
}

///
/// Allocates an input.
///
/// Transpiles from input declarations.
///
pub fn allocate_input<S, F>(
    mut system: S,
    input: F,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
    F: FnOnce() -> Result<Fr, SynthesisError>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "number_alloc"), input)?;
    number.inputize(system.namespace(|| "inputize"))?;
    let bits = utils::into_bits_le_fixed(
        system.namespace(|| "into_bits_le_fixed"),
        &number,
        bitlength,
    )?;
    Ok((number, bits))
}

///
/// Allocates a witness.
///
/// Transpiles from witness declarations.
///
pub fn allocate_witness<S, F>(
    mut system: S,
    witness: F,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
    F: FnOnce() -> Result<Fr, SynthesisError>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "number_alloc"), witness)?;
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
pub fn or<S>(system: S, a: &Boolean, b: &Boolean) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::and(system, &a.not(), &b.not())?.not())
}

///
/// The XOR logical function.
///
/// Transpiles from:
/// `{identifier} ^^ {identifier}`
///
pub fn xor<S>(system: S, a: &Boolean, b: &Boolean) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::xor(system, a, b)?)
}

///
/// The AND logical function.
///
/// Transpiles from:
/// `{identifier} && {identifier}`
///
pub fn and<S>(system: S, a: &Boolean, b: &Boolean) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::and(system, a, b)?)
}

///
/// The number equality comparison function.
///
/// Transpiles from:
/// `{identifier} == {identifier}`
///
pub fn equals_number<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::from(AllocatedNum::equals(
        system.namespace(|| "equals_number"),
        a,
        b,
    )?))
}

///
/// The boolean equality comparison function.
///
/// Transpiles from:
/// `{identifier} == {identifier}`
///
pub fn equals_boolean<S>(mut system: S, a: &Boolean, b: &Boolean) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::from(AllocatedBit::alloc(
        system.namespace(|| "equals_boolean"),
        Some(a.get_value() == b.get_value()),
    )?))
}

///
/// The number non-equality comparison function.
///
/// Transpiles from:
/// `{identifier} != {identifier}`
///
pub fn not_equals_number<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::from(AllocatedNum::equals(
        system.namespace(|| "not_equals_number"),
        a,
        b,
    )?)
    .not())
}

///
/// The boolean non-equality comparison function.
///
/// Transpiles from:
/// `{identifier} == {identifier}`
///
pub fn not_equals_boolean<S>(
    mut system: S,
    a: &Boolean,
    b: &Boolean,
) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(Boolean::from(AllocatedBit::alloc(
        system.namespace(|| "not_equals_boolean"),
        Some(a.get_value() != b.get_value()),
    )?))
}

///
/// The greater-or-equality comparison function.
///
/// Transpiles from:
/// `{identifier} >= {identifier}`
///
pub fn greater_equals<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
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

    let (diff_a_b, diff_bits) = subtract(
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
pub fn lesser_equals<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
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

    let (diff_b_a, diff_bits) = subtract(
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
pub fn greater<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    let (diff_a_b, diff_bits) =
        subtract(system.namespace(|| "greater_subtraction"), a, b, bitlength)?;

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
pub fn lesser<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    let (diff_b_a, diff_bits) =
        subtract(system.namespace(|| "lesser_subtraction"), b, a, bitlength)?;

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
pub fn add<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        let mut sum = a.get_value().unwrap();
        sum.add_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    system.enforce(
        || "enforce",
        |lc| lc + a.get_variable() + b.get_variable(),
        |lc| lc + S::one(),
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
pub fn subtract<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        let mut sum = a.get_value().unwrap();
        sum.sub_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    system.enforce(
        || "enforce",
        |lc| lc + a.get_variable() - b.get_variable(),
        |lc| lc + S::one(),
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
pub fn multiply<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
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
/// The division function.
///
/// Transpiles from:
/// `{identifier} / {identifier}`
///
pub fn divide<S>(
    mut system: S,
    _a: &AllocatedNum<Bn256>,
    _b: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        Ok(Fr::from_str("0").expect("Always valid"))
    })?;
    let bits = number.into_bits_le(system.namespace(|| "into_bits_le"))?;
    Ok((number, bits))
}

///
/// The remainder function.
///
/// Transpiles from:
/// `{identifier} % {identifier}`
///
pub fn modulo<S>(
    mut system: S,
    _a: &AllocatedNum<Bn256>,
    _b: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        Ok(Fr::from_str("0").expect("Always valid"))
    })?;
    let bits = number.into_bits_le(system.namespace(|| "into_bits_le"))?;
    Ok((number, bits))
}

///
/// The casting function.
///
/// Transpiles from:
/// `{identifier} as {type}`
///
pub fn cast<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    _bitlength: usize,
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    AllocatedNum::alloc(system.namespace(|| "alloc"), || Ok(a.get_value().unwrap()))
}

///
/// The negation function.
///
/// Transpiles from:
/// `-{identifier}`
///
pub fn negate<S>(
    mut system: S,
    a: &AllocatedNum<Bn256>,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| "alloc"), || {
        let mut negated = Fr::zero();
        negated.sub_assign(&a.get_value().unwrap());
        Ok(negated)
    })?;

    system.enforce(
        || "enforce",
        |lc| lc + a.get_variable(),
        |lc| lc - S::one(),
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
pub fn not<S>(_system: S, a: &Boolean) -> Result<Boolean, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    Ok(a.not())
}

///
/// The require function.
///
/// Transpiles from:
/// `require({expression}, {string});`
///
pub fn require<S>(mut system: S, value: &Boolean, annotation: &str)
where
    S: ConstraintSystem<Bn256>,
{
    system.enforce(
        || annotation,
        |_| value.lc(S::one(), Fr::one()),
        |lc| lc + S::one(),
        |lc| lc + S::one(),
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
pub fn conditional<S>(
    mut cs: S,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    condition: &Boolean,
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    S: ConstraintSystem<Bn256>,
{
    AllocatedNum::conditionally_select(cs.namespace(|| "conditionally_select"), a, b, condition)
}
