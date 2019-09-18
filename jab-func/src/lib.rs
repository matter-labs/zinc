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
/// The addition function.
///
/// Transpiles from:
/// `a + b`
///
pub fn sum<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let sum = AllocatedNum::alloc(system.namespace(|| name), || {
        let mut sum = a.get_value().unwrap();
        sum.add_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    let enforce_name = format!("enforce_sum_{}", name);
    system.enforce(
        || enforce_name,
        |lc| lc + a.get_variable() + b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + sum.get_variable(),
    );
    let bits_name = format!("sum_into_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| bits_name), &sum, bitlength)?;

    Ok((sum, bits))
}

///
/// The subtraction function.
///
/// Transpiles from:
/// `a - b`
///
pub fn diff<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let diff = AllocatedNum::alloc(system.namespace(|| name), || {
        let mut sum = a.get_value().unwrap();
        sum.sub_assign(&b.get_value().unwrap());
        Ok(sum)
    })?;

    let enforce_name = format!("enforce_diff_{}", name);
    system.enforce(
        || enforce_name,
        |lc| lc + a.get_variable() - b.get_variable(),
        |lc| lc + CS::one(),
        |lc| lc + diff.get_variable(),
    );
    let bits_name = format!("diff_into_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| bits_name), &diff, bitlength)?;

    Ok((diff, bits))
}

///
/// The multiplication function.
///
/// Transpiles from:
/// `a * b`
///
pub fn mul<CS>(
    mut system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    name: &str,
    bitlength: usize,
) -> Result<(AllocatedNum<Bn256>, Vec<Boolean>), SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    let mul = AllocatedNum::alloc(system.namespace(|| name), || {
        let mut mul = a.get_value().unwrap();
        mul.mul_assign(&b.get_value().unwrap());
        Ok(mul)
    })?;

    let enforce_name = format!("enforce_mul_{}", name);
    system.enforce(
        || enforce_name,
        |lc| lc + a.get_variable(),
        |lc| lc + b.get_variable(),
        |lc| lc + mul.get_variable(),
    );
    let bits_name = format!("mul_into_bits_{}", name);
    let bits = auxiliary::into_bits_le_fixed(system.namespace(|| bits_name), &mul, bitlength)?;

    Ok((mul, bits))
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
    let diff_name = format!("diff_{}", name);
    let (diff_a_b, diff_bits) = diff(system.namespace(|| "a-b"), a, b, &diff_name, bitlength)?;
    let diff_a_b_repacked =
        auxiliary::pack_bits_to_element(system.namespace(|| "pack_a-b_bits"), &diff_bits)?;
    let is_a_geq_b = Boolean::from(AllocatedNum::equals(
        system.namespace(|| "diff_equal_to_repacked"),
        &diff_a_b,
        &diff_a_b_repacked,
    )?);
    Ok(is_a_geq_b)
}

///
/// The lesser comparison function.
///
/// Transpiles from:
/// `a < b`
///
pub fn lesser<CS>(
    system: CS,
    a: &AllocatedNum<Bn256>,
    b: &AllocatedNum<Bn256>,
    name: &str,
    bitlength: usize,
) -> Result<Boolean, SynthesisError>
where
    CS: ConstraintSystem<Bn256>,
{
    greater(system, b, a, name, bitlength)
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
