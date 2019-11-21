use num_bigint::BigInt;
use std::ops::Div;
use num_traits::Signed;

/// Euclidean division of BigInt.
///
/// div_rem(9, 4) -> (2, 1)
/// div_rem(9, -4) -> (-2, 1)
/// div_rem(-9, 4) -> (-3, 3)
/// div_rem(-9, -4) -> (3, 3)
pub fn euclidean_div_rem(nominator: &BigInt, denominator: &BigInt) -> (BigInt, BigInt) {
    let mut div = nominator.div(denominator);

    if div.clone() * denominator.clone() > nominator.clone() {
        if denominator.is_positive() {
            div -= BigInt::from(1);
        } else {
            div += BigInt::from(1);
        }
    }

    let rem = nominator - div.clone() * denominator;

    (div, rem)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_div_rem() {
        let (d, r) = euclidean_div_rem(&BigInt::from(9), &BigInt::from(4));
        assert_eq!(d, BigInt::from(2));
        assert_eq!(r, BigInt::from(1));

        let (d, r) = euclidean_div_rem(&BigInt::from(-9), &BigInt::from(-4));
        assert_eq!(d, BigInt::from(3));
        assert_eq!(r, BigInt::from(3));

        let (d, r) = euclidean_div_rem(&BigInt::from(-9), &BigInt::from(4));
        assert_eq!(d, BigInt::from(-3));
        assert_eq!(r, BigInt::from(3));

        let (d, r) = euclidean_div_rem(&BigInt::from(9), &BigInt::from(-4));
        assert_eq!(d, BigInt::from(-2));
        assert_eq!(r, BigInt::from(1));
    }
}
