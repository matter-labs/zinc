mod bigint;
pub mod dummy_constraint_system;
mod fr_bigint;

pub use bigint::*;
pub use fr_bigint::*;

#[allow(dead_code)]
pub fn tree_height(size: usize) -> usize {
    let mut height = 0;
    let mut n = size;
    while n > 1 {
        n /= 2;
        height += 1;
    }

    if size > (1 << height) {
        height += 1;
    }

    height
}
