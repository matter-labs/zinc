//!
//! The Compiler macro test.
//!

use compiler::circuit;

fn main() {
    circuit!(
        inputs {
            a: uint128;
            b: field;
            c: bool;
        }

        witness {
            a: uint128;
            b: field;
            c: bool;
        }
    );
}
