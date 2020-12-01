//!
//! The operator instructions overflow tests.
//!

use num::BigInt;
use num::One;

use zinc_types::Add;
use zinc_types::Push;
use zinc_types::Sub;

use crate::error::Error;
use crate::tests::TestRunner;
use crate::tests::TestingError;

#[test]
fn unsigned_positive_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(
            BigInt::from(255),
            zinc_types::IntegerType::U8.into(),
        ))
        .push(Push::new(BigInt::one(), zinc_types::IntegerType::U8.into()))
        .push(Add)
        .test(&[256]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::Error(Error::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_negative_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(
            BigInt::from(254),
            zinc_types::IntegerType::U8.into(),
        ))
        .push(Push::new(
            BigInt::from(255),
            zinc_types::IntegerType::U8.into(),
        ))
        .push(Sub)
        .test(&[-1]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::Error(Error::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_positive_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(
            BigInt::from(127),
            zinc_types::IntegerType::I8.into(),
        ))
        .push(Push::new(BigInt::one(), zinc_types::IntegerType::I8.into()))
        .push(Add)
        .test(&[128]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::Error(Error::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_negative_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(
            BigInt::from(-128),
            zinc_types::IntegerType::I8.into(),
        ))
        .push(Push::new(BigInt::one(), zinc_types::IntegerType::I8.into()))
        .push(Sub)
        .test(&[-129]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::Error(Error::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_positive_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(
            BigInt::from(254),
            zinc_types::IntegerType::U8.into(),
        ))
        .push(Push::new(BigInt::one(), zinc_types::IntegerType::U8.into()))
        .push(Add)
        .test(&[255])
}

#[test]
fn unsigned_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(
            BigInt::from(255),
            zinc_types::IntegerType::U8.into(),
        ))
        .push(Push::new(
            BigInt::from(255),
            zinc_types::IntegerType::U8.into(),
        ))
        .push(Sub)
        .test(&[0])
}

#[test]
fn signed_positive_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(
            BigInt::from(126),
            zinc_types::IntegerType::I8.into(),
        ))
        .push(Push::new(BigInt::one(), zinc_types::IntegerType::I8.into()))
        .push(Add)
        .test(&[127])
}

#[test]
fn signed_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(
            BigInt::from(-127),
            zinc_types::IntegerType::I8.into(),
        ))
        .push(Push::new(BigInt::one(), zinc_types::IntegerType::I8.into()))
        .push(Sub)
        .test(&[-128])
}
