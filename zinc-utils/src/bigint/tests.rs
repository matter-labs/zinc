//!
//! The BigInt parsing tests.
//!

use num::BigInt;
use num::One;
use num::Zero;

use crate::bigint;
use crate::bigint::error::Error;

#[test]
fn ok_from_str() {
    assert_eq!(bigint::from_str("0"), Ok(BigInt::zero()));
    assert_eq!(bigint::from_str("0E0"), Ok(BigInt::zero()));
    assert_eq!(bigint::from_str("0.0E0"), Ok(BigInt::zero()));
    assert_eq!(bigint::from_str("0E10"), Ok(BigInt::zero()));
    assert_eq!(bigint::from_str("0.0E10"), Ok(BigInt::zero()));
    assert_eq!(bigint::from_str("1"), Ok(BigInt::one()));
    assert_eq!(bigint::from_str("1E0"), Ok(BigInt::one()));
    assert_eq!(bigint::from_str("1.0E0"), Ok(BigInt::one()));
    assert_eq!(
        bigint::from_str("1.0E9"),
        Ok(BigInt::from(1_000_000_000_u64)),
    );
    assert_eq!(bigint::from_str("42.666_E3"), Ok(BigInt::from(42_666_u64)));
    assert_eq!(
        bigint::from_str("42.666_E6"),
        Ok(BigInt::from(42_666_000_u64)),
    );
    assert_eq!(bigint::from_str("0.001_E3"), Ok(BigInt::one()));
    assert_eq!(
        bigint::from_str("0.000_001_E9"),
        Ok(BigInt::from(1_000_u64)),
    );
    assert_eq!(
        bigint::from_str("1.000_001_E9"),
        Ok(BigInt::from(1_000_001_000_u64)),
    );
    assert_eq!(
        bigint::from_str("0.000_52_E9"),
        Ok(BigInt::from(520_000_u64)),
    );
    assert_eq!(
        bigint::from_str("0.000_000_000_000_000_001_E18"),
        Ok(BigInt::one()),
    );
}

#[test]
fn error_number_parsing() {
    assert!(matches!(
        bigint::from_str("42a.3E6"),
        Err(Error::NumberParsing(_))
    ));
}

#[test]
fn error_exponent_parsing() {
    assert!(matches!(
        bigint::from_str("42.0Ea"),
        Err(Error::ExponentParsing(_))
    ));
}

#[test]
fn error_exponent_too_small() {
    assert!(matches!(
        bigint::from_str("42.666_E2"),
        Err(Error::ExponentTooSmall(2))
    ));
    assert!(matches!(
        bigint::from_str("42.001E2"),
        Err(Error::ExponentTooSmall(2))
    ));
}
