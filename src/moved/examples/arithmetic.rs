use jab::prelude::*;

fn main() {

    let xor: BellmanCircuit = circuit! {
        inputs { 
            r: uint128;
        };

        witness {
            a: uint128;
            b: uint128;
        };

        let x: uint128 = 3 * a + b - 7 * (a * b + 2);
        let y: uint128 = (x + 9) * a;
        require(r == y);

    }?;

}