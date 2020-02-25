use crate::gadgets::Scalar;
use crate::{Engine, Result};
use bellman::ConstraintSystem;
use franklin_crypto::circuit::ecc::{EdwardsPoint};
use franklin_crypto::circuit::baby_eddsa::EddsaSignature;
use franklin_crypto::jubjub::{JubjubParams, FixedGenerators};
use crate::core::EvaluationStack;
use crate::gadgets::stdlib::NativeFunction;
use franklin_crypto::jubjub::edwards::Point;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::bellman::SynthesisError;

pub struct VerifyEddsaSignature;

impl<E: Engine> NativeFunction<E> for VerifyEddsaSignature {
    fn execute<CS>(&self, mut cs: CS, stack: &mut EvaluationStack<E>) -> Result
    where
        CS: ConstraintSystem<E>
    {
        let r_x = stack.pop()?.value()?.to_number(cs.namespace(|| "to_number r_x"))?;
        let r_y = stack.pop()?.value()?.to_number(cs.namespace(|| "to_number r_y"))?;
        let s = stack.pop()?.value()?.to_number(cs.namespace(|| "to_number s"))?;
        let pk_x = stack.pop()?.value()?.to_number(cs.namespace(|| "to_number pk_x"))?;
        let pk_y = stack.pop()?.value()?.to_number(cs.namespace(|| "to_number pk_y"))?;
        let message = stack.pop()?.value()?;

        let r = edwards_point_from_witness(cs.namespace(|| "r"), r_x, r_y, E::jubjub_params())?;
        let pk = edwards_point_from_witness(cs.namespace(|| "pk"), pk_x, pk_y, E::jubjub_params())?;

        let signature = EddsaSignature { r, s, pk };

        let is_valid = verify_signature(
            cs.namespace(|| "verify_signature"),
            &message,
            &signature,
            E::jubjub_params()
        )?;

        stack.push(is_valid.into())
    }
}

pub fn verify_signature<E, CS>(
    mut cs: CS,
    message: &Scalar<E>,
    signature: &EddsaSignature<E>,
    params: &E::Params,
) -> Result<Scalar<E>>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    let message_bits = message
        .to_expression::<CS>()
        .into_bits_le_strict(cs.namespace(|| "into_bits"))?;

    let public_generator = params.generator(FixedGenerators::SpendingKeyGenerator).clone();
    let generator = EdwardsPoint::witness(
        cs.namespace(|| "allocate public generator"),
        Some(public_generator),
        params,
    )?;

    let is_verified = signature.is_verified_raw_message_signature(
        cs.namespace(|| "is_verified_signature"),
        params,
        &message_bits,
        generator,
        32,
    )?;

    Scalar::from_boolean(cs.namespace(|| "from_boolean"), is_verified)
}

fn edwards_point_from_witness<E, CS>(
    mut cs: CS,
    x: AllocatedNum<E>,
    y: AllocatedNum<E>,
    params: &E::Params,
) -> std::result::Result<EdwardsPoint<E>, SynthesisError>
    where
        E: Engine,
        CS: ConstraintSystem<E>
{
    let point = match (x.get_value(), y.get_value()) {
        (Some(xv), Some(yv)) => {
            Point::from_xy(xv, yv, params)
        }
        _ => None,
    };

    let edwards_point = EdwardsPoint::witness(
        cs.namespace(|| "edwards point"),
        point,
        params
    )?;

    cs.enforce(
        || "x",
        |zero| zero + x.get_variable(),
        |zero| zero + CS::one(),
        |zero| zero + edwards_point.get_x().get_variable(),
    );

    cs.enforce(
        || "y",
        |zero| zero + y.get_variable(),
        |zero| zero + CS::one(),
        |zero| zero + edwards_point.get_y().get_variable(),
    );

    Ok(edwards_point)
}
