//!
//! The miscellaneous math functions.
//!

#[cfg(test)]
mod tests;

///
/// The power of two enough to represent `n`.
///
pub fn log2ceil(n: usize) -> usize {
    let mut log = 0;
    while n > (1 << log) {
        log += 1;
    }
    log
}

///
/// The closest power of two that is equal or less than `n`.
///
pub fn floor_to_power_of_two(n: usize) -> usize {
    let mut pow = 0;
    while n >= 1 << (pow + 1) {
        pow += 1;
    }
    1 << pow
}
