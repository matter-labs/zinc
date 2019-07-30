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

// Expands into:

/*

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