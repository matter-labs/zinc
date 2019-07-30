/*
use jab::prelude::*;

fn main() {

    let xor: BellmanCircuit = circuit! {
        inputs {
            c: bool;
        };

        witness {
            a: bool;
            b: bool;
        };

        require(c == a && b);
    }?;

}
*/

use crate::gadgets::*;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use ff::Field;
use franklin_crypto::circuit::boolean::{AllocatedBit, Boolean};
use pairing::bn256::{Bn256, Fr};

pub struct XorCircuit {
    //public input
    pub c: Option<bool>,

    //witness input
    pub a: Option<bool>,
    pub b: Option<bool>,
}

impl Circuit<Bn256> for XorCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let a_bit = AllocatedBit::alloc(cs.namespace(|| "allocate bit a"), self.a)?;
        let a = Boolean::from(a_bit);

        let b_bit = AllocatedBit::alloc(cs.namespace(|| "allocate bit b"), self.b)?;
        let b = Boolean::from(b_bit);

        let c_bit = AllocatedBit::alloc(cs.namespace(|| "allocate bit a"), self.c)?;
        inputize_bool(cs.namespace(|| "inputize bool a"), &c_bit)?;
        let c = Boolean::from(c_bit);
        let xor = Boolean::xor(cs.namespace(|| "xor a b"), &a, &b)?;

        cs.enforce(
            || "require c == xor a b",
            |_| c.lc(CS::one(), Fr::one()),
            |lc| lc + CS::one(),
            |_| xor.lc(CS::one(), Fr::one()),
        );
        Ok(())
    }
}

