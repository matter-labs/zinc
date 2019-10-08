#![allow(unused_imports)]

use r1cs::Bn256;
use r1cs::Circuit;
use r1cs::Fr;
use r1cs::TestConstraintSystem;

fn main() {
    let mut system = TestConstraintSystem::<Bn256>::new();

    let circuit = circuit::GeneratedCircuit::default();
    circuit.synthesize(&mut system).expect("Synthesis failed");

    dbg!(system.find_unconstrained());
    dbg!(system.num_constraints());
    dbg!(system.num_inputs());

    if let Some(token) = system.which_is_unsatisfied() {
        println!("Error: require statement '{}' is unsatisfied", token);
    }
}
