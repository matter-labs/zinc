use crate::core::EvaluationStack;
use crate::gadgets::stdlib::NativeFunction;
use crate::gadgets::Scalar;
use crate::{Engine, Result};
use bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;
use franklin_crypto::circuit::baby_eddsa::EddsaSignature;
use franklin_crypto::circuit::ecc::EdwardsPoint;
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::jubjub::edwards::Point;
use franklin_crypto::jubjub::{FixedGenerators, JubjubParams};

pub struct VerifyEddsaSignature;

impl<E: Engine> NativeFunction<E> for VerifyEddsaSignature {
    fn execute<CS>(&self, mut cs: CS, stack: &mut EvaluationStack<E>) -> Result
    where
        CS: ConstraintSystem<E>,
    {
        let r_x = stack
            .pop()?
            .value()?
            .to_number(cs.namespace(|| "to_number r_x"))?;
        let r_y = stack
            .pop()?
            .value()?
            .to_number(cs.namespace(|| "to_number r_y"))?;
        let s = stack
            .pop()?
            .value()?
            .to_number(cs.namespace(|| "to_number s"))?;
        let pk_x = stack
            .pop()?
            .value()?
            .to_number(cs.namespace(|| "to_number pk_x"))?;
        let pk_y = stack
            .pop()?
            .value()?
            .to_number(cs.namespace(|| "to_number pk_y"))?;
        let message = stack.pop()?.value()?;

        let r = edwards_point_from_witness(cs.namespace(|| "r"), r_x, r_y, E::jubjub_params())?;
        let pk = edwards_point_from_witness(cs.namespace(|| "pk"), pk_x, pk_y, E::jubjub_params())?;

        let signature = EddsaSignature { r, s, pk };

        let is_valid = verify_signature(
            cs.namespace(|| "verify_signature"),
            &message,
            &signature,
            E::jubjub_params(),
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
    CS: ConstraintSystem<E>,
{
    let message_bits = message
        .to_expression::<CS>()
        .into_bits_le_strict(cs.namespace(|| "into_bits"))?;

    let public_generator = params
        .generator(FixedGenerators::SpendingKeyGenerator)
        .clone();
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
    CS: ConstraintSystem<E>,
{
    let point = match (x.get_value(), y.get_value()) {
        (Some(xv), Some(yv)) => Point::from_xy(xv, yv, params),
        _ => None,
    };

    let edwards_point = EdwardsPoint::witness(cs.namespace(|| "edwards point"), point, params)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    use ff::{Field, PrimeField};
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use pairing::bn256::{Bn256, Fr};
    use zinc_bytecode::scalar::ScalarType;

    #[test]
    fn test_verify() -> Result {
        let r_x = Fr::from_str(
            "13640612427693488274999841050634523339358198536154728841267323157043880421621",
        )
        .unwrap();
        let r_y = Fr::from_str(
            "9509884871693549865753143729935660249535333730208041183969775141915970240099",
        )
        .unwrap();
        let s = Fr::from_str(
            "494745623983833019655061946093744216550252666011167101498285355927842221703",
        )
        .unwrap();
        let pk_x = Fr::from_str(
            "20453034254071666356681228067672474579643265895584845472570305237276758169245",
        )
        .unwrap();
        let pk_y = Fr::from_str(
            "20956838306014746826052367476917828000427140731634825069188146376965741319115",
        )
        .unwrap();
        let message = Fr::from_str("72034994866411393714850512529618071366").unwrap();

        let mut stack = EvaluationStack::<Bn256>::new();

        stack.push(Scalar::new_unchecked_constant(message, ScalarType::Field).into())?;
        stack.push(Scalar::new_unchecked_constant(pk_y, ScalarType::Field).into())?;
        stack.push(Scalar::new_unchecked_constant(pk_x, ScalarType::Field).into())?;
        stack.push(Scalar::new_unchecked_constant(s, ScalarType::Field).into())?;
        stack.push(Scalar::new_unchecked_constant(r_y, ScalarType::Field).into())?;
        stack.push(Scalar::new_unchecked_constant(r_x, ScalarType::Field).into())?;

        let mut cs = TestConstraintSystem::new();
        VerifyEddsaSignature.execute(cs.namespace(|| "signature check"), &mut stack)?;

        let is_valid = stack.pop()?.value()?;

        assert_eq!(is_valid.get_value(), Some(Fr::one()));
        assert!(cs.is_satisfied(), "unsatisfied");
        assert_eq!(cs.which_is_unsatisfied(), None, "unconstrained");

        Ok(())
    }
}
