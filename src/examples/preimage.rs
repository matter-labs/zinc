use jab::prelude::*;

fn main() {

    // prove that we know an sha256 preimage
    let xor: BellmanCircuit = circuit! {
        inputs { 
            hash: uint253;
        };

        witness {
            preimage: uint253;
        };

        let hash_bits: memory_vector<bool> = sha256(preimage);
        let truncated: memory_vector<bool> = hash_bits.take(253);
        let output: uint253 = truncated.pack();
        require(hash == output);
    }?;

}