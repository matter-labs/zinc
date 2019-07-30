use jab::prelude::*;

fn main() {

    let xor: BellmanCircuit = circuit! {
        inputs { 
            sum: uint128;
            triple_max: uint128;
        };

        witness {
            a: uint128;
            b: uint128;
        };

        let s: uint128 = a + b;
        require(sum == s);

        let mut m: uint128 = 0;

        if a >= b {
            m = a;
        } else {
            m = b;
        }

        require(triple_max == m * 3);
    }?;

}