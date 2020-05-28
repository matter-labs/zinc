//!
//! The Euclidean division and remainder.
//!

use std::ops::Div;

use num_bigint::BigInt;
use num_traits::Signed;
use num_traits::Zero;

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
            div -= BigInt::from(1);
        } else {
            div += BigInt::from(1);
        }
    }

    let rem = nominator - div.clone() * denominator;

    Some((div, rem))
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;

    use super::div_rem;

    #[test]
    fn test_div_rem() {
        let (d, r) = div_rem(&BigInt::from(9), &BigInt::from(4)).unwrap();
        assert_eq!(d, BigInt::from(2));
        assert_eq!(r, BigInt::from(1));

        let (d, r) = div_rem(&BigInt::from(-9), &BigInt::from(-4)).unwrap();
        assert_eq!(d, BigInt::from(3));
        assert_eq!(r, BigInt::from(3));

        let (d, r) = div_rem(&BigInt::from(-9), &BigInt::from(4)).unwrap();
        assert_eq!(d, BigInt::from(-3));
        assert_eq!(r, BigInt::from(3));

        let (d, r) = div_rem(&BigInt::from(9), &BigInt::from(-4)).unwrap();
        assert_eq!(d, BigInt::from(-2));
        assert_eq!(r, BigInt::from(1));
    }
}
