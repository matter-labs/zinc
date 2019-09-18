use bellman::Circuit;
use pairing::bn256::Bn256;
use sapling_crypto::circuit::test::TestConstraintSystem;

use circuit::GeneratedCircuit;

fn main() {
    let mut system = TestConstraintSystem::<Bn256>::new();

    let circuit = GeneratedCircuit::default();
    circuit.synthesize(&mut system).expect("Synthesis failed");

    dbg!(system.find_unconstrained());
    dbg!(system.num_constraints());
    dbg!(system.num_inputs());

    if let Some(token) = system.which_is_unsatisfied() {
        eprintln!("Error: {} is unsatisfied", token);
    }
}
