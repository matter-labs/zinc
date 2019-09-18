use bellman::Circuit;
use bellman::ConstraintSystem;
use bellman::SynthesisError;
use ff::Field;
use ff::PrimeField;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;

#[derive(Default)]
pub struct GeneratedCircuit {}
impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {
        let a = jab::alloc_input(&mut cs, || Ok(Fr::zero()), "a", 8)?.0;
        let x = jab::alloc_input(&mut cs, || Ok(Fr::zero()), "x", 8)?.0;

        let b = jab::alloc_witness(&mut cs, || Ok(Fr::zero()), "b", 8)?.0;

        let id_000001 = AllocatedNum::alloc(cs.namespace(|| "id_000001"), || Ok(Fr::from_str("2").unwrap()))?;
        let id_000002 = jab::multiplication(&mut cs, &id_000001, &a, "id_000002", 254)?.0;

        let id_000003 = AllocatedNum::alloc(cs.namespace(|| "id_000003"), || Ok(Fr::from_str("15").unwrap()))?;
        let id_000004 = jab::addition(&mut cs, &b, &id_000003, "id_000004", 254)?.0;

        let id_000005 = AllocatedNum::alloc(cs.namespace(|| "id_000005"), || Ok(Fr::from_str("64").unwrap()))?;
        let id_000006 = jab::subtraction(&mut cs, &id_000004, &id_000005, "id_000006", 254)?.0;

        let id_000007 = AllocatedNum::alloc(cs.namespace(|| "id_000007"), || Ok(Fr::from_str("3").unwrap()))?;
        let id_000008 = jab::multiplication(&mut cs, &id_000007, &id_000006, "id_000008", 254)?.0;

        let id_000009 = jab::addition(&mut cs, &id_000002, &id_000008, "id_000009", 254)?.0;

        let id_000010 = AllocatedNum::alloc(cs.namespace(|| "id_000010"), || Ok(Fr::from_str("25").unwrap()))?;
        let id_000011 = jab::multiplication(&mut cs, &id_000010, &x, "id_000011", 254)?.0;

        let id_000012 = jab::subtraction(&mut cs, &id_000009, &id_000011, "id_000012", 254)?.0;

        let id_000013 = {
            let id_000014 = {
                let id_000015 = AllocatedNum::alloc(cs.namespace(|| "id_000015"), || Ok(Fr::from_str("4").unwrap()))?;
                let id_000016 = AllocatedNum::alloc(cs.namespace(|| "id_000016"), || Ok(Fr::from_str("5").unwrap()))?;
                let id_000017 = jab::addition(&mut cs, &id_000016, &id_000015, "id_000017", 254)?.0;

                id_000017
            };
            let id_000018 = AllocatedNum::alloc(cs.namespace(|| "id_000018"), || Ok(Fr::from_str("2").unwrap()))?;
            let id_000019 = jab::addition(&mut cs, &id_000018, &id_000014, "id_000019", 254)?.0;

            id_000019
        };
        let id_000020 = jab::addition(&mut cs, &id_000012, &id_000013, "id_000020", 254)?.0;

        let c = id_000020;

        let id_000021 = AllocatedNum::alloc(cs.namespace(|| "id_000021"), || Ok(Fr::from_str("136").unwrap()))?;
        let d = id_000021;

        let id_000022 = jab::negation(&mut cs, &d, "id_000022", 254)?.0;

        let id_000023 = jab::equals(&mut cs, &c, &id_000022, "id_000023", 254)?;

        let cond = id_000023;

        dbg!(c.get_variable());

        let id_000024 = Boolean::constant(true);
        let id_000025 = jab::or(&mut cs, &cond, &id_000024, "id_000025")?;

        jab::require(&mut cs, &id_000025, "require_test");

        for i in 0..10 {
            let iter_name = format!("id_000026_{}", i);
            let i = AllocatedNum::alloc(cs.namespace(|| iter_name), || Ok(Fr::from_str(i.to_string().as_str()).unwrap()))?;
            dbg!(i.get_variable());

        }

        Ok(())
    }
}
