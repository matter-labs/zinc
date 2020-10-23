//!
//! The Euclidean division and remainder tests.
//!

use num::BigInt;

use crate::euclidean;

#[test]
fn ok_div_rem() {
    let (d, r) = euclidean::div_rem(&BigInt::from(9), &BigInt::from(4))
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_eq!(d, BigInt::from(2));
    assert_eq!(r, BigInt::from(1));

    let (d, r) = euclidean::div_rem(&BigInt::from(-9), &BigInt::from(-4))
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_eq!(d, BigInt::from(3));
    assert_eq!(r, BigInt::from(3));

    let (d, r) = euclidean::div_rem(&BigInt::from(-9), &BigInt::from(4))
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_eq!(d, BigInt::from(-3));
    assert_eq!(r, BigInt::from(3));

    let (d, r) = euclidean::div_rem(&BigInt::from(9), &BigInt::from(-4))
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_eq!(d, BigInt::from(-2));
    assert_eq!(r, BigInt::from(1));
}
