//!
//! The Euclidean division and remainder.
//!

#[cfg(test)]
mod tests;

use std::ops::Div;

use num::BigInt;
use num::One;
use num::Signed;
use num::Zero;

///
/// Euclidean division of BigInt.
///
/// div_rem(9, 4) -> (2, 1)
/// div_rem(9, -4) -> (-2, 1)
/// div_rem(-9, 4) -> (-3, 3)
/// div_rem(-9, -4) -> (3, 3)
pub fn div_rem(nominator: &BigInt, denominator: &BigInt) -> Option<(BigInt, BigInt)> {
    if denominator.is_zero() {
        return None;
    }

    let mut div = nominator.div(denominator);

    if div.clone() * denominator.clone() > nominator.clone() {
        if denominator.is_positive() {
            div -= BigInt::one();
        } else {
            div += BigInt::one();
        }
    }

    let rem = nominator - div.clone() * denominator;

    Some((div, rem))
}
