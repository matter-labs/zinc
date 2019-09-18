use bellman::Circuit;
use bellman::ConstraintSystem;
use bellman::SynthesisError;
use ff::Field;
use ff::PrimeField;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;
use franklin_crypto::circuit::num::AllocatedNum;

#[derive(Default)]
pub struct GeneratedCircuit {}
impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {
        let a = jab::alloc_input(&mut cs, || Ok(Fr::zero()), "a", 8)?.0;
        let x = jab::alloc_input(&mut cs, || Ok(Fr::zero()), "x", 8)?.0;

        let b = jab::alloc_witness(&mut cs, || Ok(Fr::zero()), "b", 8)?.0;

        let temp_000001 = AllocatedNum::alloc(cs.namespace(|| "temp_000001"), || Ok(Fr::from_str("2").unwrap()))?;
        let temp_000002 = jab::multiplication(&mut cs, &a, &temp_000001, "temp_000002", 254)?.0;

        let temp_000003 = AllocatedNum::alloc(cs.namespace(|| "temp_000003"), || Ok(Fr::from_str("15").unwrap()))?;
        let temp_000004 = jab::addition(&mut cs, &temp_000003, &b, "temp_000004", 254)?.0;

        let temp_000005 = AllocatedNum::alloc(cs.namespace(|| "temp_000005"), || Ok(Fr::from_str("64").unwrap()))?;
        let temp_000006 = jab::subtraction(&mut cs, &temp_000005, &temp_000004, "temp_000006", 254)?.0;

        let temp_000007 = AllocatedNum::alloc(cs.namespace(|| "temp_000007"), || Ok(Fr::from_str("3").unwrap()))?;
        let temp_000008 = jab::multiplication(&mut cs, &temp_000006, &temp_000007, "temp_000008", 254)?.0;

        let temp_000009 = jab::addition(&mut cs, &temp_000008, &temp_000002, "temp_000009", 254)?.0;

        let temp_000010 = AllocatedNum::alloc(cs.namespace(|| "temp_000010"), || Ok(Fr::from_str("25").unwrap()))?;
        let temp_000011 = jab::multiplication(&mut cs, &x, &temp_000010, "temp_000011", 254)?.0;

        let temp_000012 = jab::subtraction(&mut cs, &temp_000011, &temp_000009, "temp_000012", 254)?.0;

        let c = temp_000012;

        let temp_000013 = AllocatedNum::alloc(cs.namespace(|| "temp_000013"), || Ok(Fr::from_str("147").unwrap()))?;
        let d = temp_000013;

        let temp_000014 = jab::negation(&mut cs, &d, "temp_000014", 254)?.0;

        let temp_000015 = jab::equals(&mut cs, &temp_000014, &c, "temp_000015", 254)?;

        let cond = temp_000015;

        dbg!(c.get_variable());

        jab::require(&mut cs, &cond, "require_test");

        Ok(())
    }
}
