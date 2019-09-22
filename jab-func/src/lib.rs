//!
//! Functions used by generated circuits directly.
//!

mod auxiliary;

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
pub fn allocation<CS>(
    mut system: CS,
    name: &str,
    number: &str,
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    AllocatedNum::alloc(system.namespace(|| name), || {
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
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
    F: FnOnce() -> Result<Fr, SynthesisError>,
{
    let number = AllocatedNum::alloc(system.namespace(|| name), input)?;
    let inputize_name = format!("inputize_{}", name);
    number.inputize(system.namespace(|| &inputize_name))?;

    let bits_name = format!("input_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| &bits_name), &number, bitlength)?;

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
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
    F: FnOnce() -> Result<Fr, SynthesisError>,
{
    let number = AllocatedNum::alloc(system.namespace(|| name), witness)?;

    let bits_name = format!("witness_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| &bits_name), &number, bitlength)?;

    Ok((number, bits))
}

///
/// The OR logical function.
///
/// Transpiles from:
/// `{identifier} || {identifier}`
///
pub fn or<CS>(system: CS, a: &Boolean, b: &Boolean, _name: &str) -> Result<Boolean, SynthesisError>
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
pub fn xor<CS>(system: CS, a: &Boolean, b: &Boolean, _name: &str) -> Result<Boolean, SynthesisError>
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
pub fn and<CS>(system: CS, a: &Boolean, b: &Boolean, _name: &str) -> Result<Boolean, SynthesisError>
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
    name: &str,
    _bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let equals_name = format!("eq_equals_{}", name);
    Ok(Boolean::from(AllocatedNum::equals(
        system.namespace(|| equals_name),
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
    name: &str,
    _bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let not_equals_name = format!("ne_equals_{}", name);
    Ok(Boolean::from(AllocatedNum::equals(
        system.namespace(|| not_equals_name),
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
    name: &str,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let equals_name = format!("ge_equals_{}", name);
    if let Boolean::Is(bit) = Boolean::from(AllocatedNum::equals(
        system.namespace(|| equals_name),
        a,
        b,
    )?) {
        if let Some(true) = bit.get_value() {
            return Ok(Boolean::from(bit));
        }
    }

    let diff_name = format!("ge_diff_{}", name);
    let (diff_a_b, diff_bits) =
        subtraction(system.namespace(|| &diff_name), a, b, &diff_name, bitlength)?;

    let repacked_name = format!("ge_repacked_{}", name);
    let diff_a_b_repacked =
        auxiliary::pack_bits_to_element(system.namespace(|| &repacked_name), &diff_bits)?;

    let equal_to_repacked_name = format!("ge_equal_to_repacked_{}", name);
    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| &equal_to_repacked_name),
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
    name: &str,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let equals_name = format!("le_equals_{}", name);
    if let Boolean::Is(bit) = Boolean::from(AllocatedNum::equals(
        system.namespace(|| equals_name),
        a,
        b,
    )?) {
        if let Some(true) = bit.get_value() {
            return Ok(Boolean::from(bit));
        }
    }

    let diff_name = format!("le_diff_{}", name);
    let (diff_b_a, diff_bits) =
        subtraction(system.namespace(|| &diff_name), b, a, &diff_name, bitlength)?;

    let repacked_name = format!("le_repacked_{}", name);
    let diff_b_a_repacked =
        auxiliary::pack_bits_to_element(system.namespace(|| &repacked_name), &diff_bits)?;

    let equal_to_repacked_name = format!("le_equal_to_repacked_{}", name);
    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| &equal_to_repacked_name),
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
    name: &str,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let diff_name = format!("gt_diff_{}", name);
    let (diff_a_b, diff_bits) =
        subtraction(system.namespace(|| &diff_name), a, b, &diff_name, bitlength)?;

    let repacked_name = format!("gt_repacked_{}", name);
    let diff_a_b_repacked =
        auxiliary::pack_bits_to_element(system.namespace(|| &repacked_name), &diff_bits)?;

    let equal_to_repacked_name = format!("gt_equal_to_repacked_{}", name);
    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| &equal_to_repacked_name),
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
    name: &str,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let diff_name = format!("lt_diff_{}", name);
    let (diff_b_a, diff_bits) =
        subtraction(system.namespace(|| &diff_name), b, a, &diff_name, bitlength)?;

    let repacked_name = format!("lt_repacked_{}", name);
    let diff_b_a_repacked =
        auxiliary::pack_bits_to_element(system.namespace(|| &repacked_name), &diff_bits)?;

    let equal_to_repacked_name = format!("lt_equal_to_repacked_{}", name);
    let result = Boolean::from(AllocatedNum::equals(
        system.namespace(|| &equal_to_repacked_name),
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
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| name), || {
        let mut sum = a.get_value().unwrap();
        sum.add_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    let enforce_name = format!("sum_enforce_{}", name);
    system.enforce(
        || enforce_name,
        |lc| lc + a.get_variable() + b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + number.get_variable(),
    );
    let bits_name = format!("sum_into_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| bits_name), &number, bitlength)?;

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
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| name), || {
        let mut sum = a.get_value().unwrap();
        sum.sub_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    let enforce_name = format!("diff_enforce_{}", name);
    system.enforce(
        || enforce_name,
        |lc| lc + a.get_variable() - b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + number.get_variable(),
    );
    let bits_name = format!("diff_into_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| bits_name), &number, bitlength)?;

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
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| name), || {
        let mut mul = a.get_value().unwrap();
        mul.mul_assign(&b.get_value().unwrap());
        Ok(mul)
    })?;

    let enforce_name = format!("mul_enforce_{}", name);
    system.enforce(
        || enforce_name,
        |lc| lc + a.get_variable(),
        |lc| lc + b.get_variable(),
        |lc| lc + number.get_variable(),
    );
    let bits_name = format!("mul_into_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| bits_name), &number, bitlength)?;

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
    name: &str,
    _bitlength: usize,
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    AllocatedNum::alloc(system.namespace(|| name), || Ok(a.get_value().unwrap()))
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
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let number = AllocatedNum::alloc(system.namespace(|| name), || {
        let mut negated = Fr::zero();
        negated.sub_assign(&a.get_value().unwrap());
        Ok(negated)
    })?;

    let enforce_name = format!("neg_enforce_{}", name);
    system.enforce(
        || enforce_name,
        |lc| lc + a.get_variable(),
        |lc| lc - CS::one(),
        |lc| lc + number.get_variable(),
    );
    let bits_name = format!("neg_into_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| bits_name), &number, bitlength)?;

    Ok((number, bits))
}

///
/// The NOT logical function.
///
/// Transpiles from:
/// `!{identifier}`
///
pub fn not<CS>(_system: CS, a: &Boolean, _name: &str) -> Result<Boolean, SynthesisError>
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
pub fn require<CS>(mut system: CS, expr: &Boolean, name: &str)
where
    CS: ConstraintSystem<Bn256>,
{
    system.enforce(
        || name,
        |_| expr.lc(CS::one(), Fr::one()),
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
    name: &str,
) -> Result<AllocatedNum<Bn256>, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    AllocatedNum::conditionally_select(cs.namespace(|| name), a, b, condition)
}
