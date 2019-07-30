use jab::prelude::*;

fn main() {

    // prove that we computed the 12th fibbonaci number correctly
    let xor: BellmanCircuit = circuit! {
        inputs { 
            fibonacci12: field;
            a: field;
            b: field;
        };

        let mut fib1: field = a;
        let mut fib2: field = b;

        for i in 3..12 {
            let next = fib1 + fib2;
            fib1 = fib2;
            fib2 = next;
        }

        require(fib2 == fibonacci12);
    }?;

}