//!
//! Functions used by generated circuits directly.
//!
//! Types
//! uintX corresponds to (AllocatedNum<Fr>, Vec<Bool>) where Vec lenth is X, field is same where X=Fr::NUM_BITS (equals 254). Those are tuples, whether number or bit representation is used depends on the operation
//! bool -> Boolean
//! Vector is generic over other described types
//! Struct consists of types described above
//!
//! Operators
//! each of operator below should have two versions, checked and unchecked
//! operation transpiles to checked version according to some logic, initially in case of explicit assignment (let c = a + b), otherwise we proceed with unchecked version to optimize
//! TODO: what if long expression overflows so much that becomes valid again, we should have some special syntax for ensuring this. Possible solution is to enforce overflow check if result is possible to be greater then 254 bit
//!

mod auxiliary;

use bellman::ConstraintSystem;
use bellman::SynthesisError;
use ff::Field;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

///
/// Describes an input.
///
/// Transpiles from:
/// inputs {
///     {identifier}: {type};
/// }
///
pub fn alloc_input<CS, F>(
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
/// Transpiles from:
/// witness {
///     {identifier}: {type};
/// }
///
pub fn alloc_witness<CS, F>(
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
/// The equals comparison function.
///
/// Transpiles from:
/// `a == b`
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
/// The not equals comparison function.
///
/// Transpiles from:
/// `a != b`
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
    )?).not())
}

///
/// The greater equals comparison function.
///
/// Transpiles from:
/// `a >= b`
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
    let (diff_a_b, diff_bits) = subtraction(system.namespace(|| &diff_name), a, b, &diff_name, bitlength)?;

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
/// The lesser equals comparison function.
///
/// Transpiles from:
/// `a <= b`
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
    let (diff_b_a, diff_bits) = subtraction(system.namespace(|| &diff_name), b, a, &diff_name, bitlength)?;

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
/// `a > b`
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
    let (diff_a_b, diff_bits) = subtraction(system.namespace(|| &diff_name), a, b, &diff_name, bitlength)?;

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
/// `a < b`
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
    let (diff_b_a, diff_bits) = subtraction(system.namespace(|| &diff_name), b, a, &diff_name, bitlength)?;

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
/// `a + b`
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
/// `a - b`
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
/// `a * b`
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
/// The negation function.
///
/// Transpiles from:
/// `-a`
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
/// The not logical function.
///
/// Transpiles from:
/// `!a`
///
pub fn not<CS>(
    _system: CS,
    a: &Boolean,
    _name: &str,
) -> Result<Boolean, SynthesisError>
    where
        CS: ConstraintSystem<Bn256>,
{
    Ok(a.not())
}

///
/// The require function.
///
/// Transpiles from:
/// `require(expr, annotation);`
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
///     a = b
/// } else {
///     a = c
/// }
///
pub fn if_else_example<CS: ConstraintSystem<Bn256>>(
    mut cs: CS,
    a: AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    c: &AllocatedNum<Bn256>,
    cond: &Boolean,
) -> Result<(), SynthesisError> {
    AllocatedNum::conditionally_select(cs.namespace(|| "select_first_if"), &a, b, cond)?;
    AllocatedNum::conditionally_select(cs.namespace(|| "select_first_else"), &a, c, &cond.not())?;
    Ok(())
}
