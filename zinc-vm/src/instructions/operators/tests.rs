//!
//! The operator instructions overflow tests.
//!

use num_bigint::BigInt;
use num_traits::One;

use zinc_build::Add;
use zinc_build::IntegerType;
use zinc_build::Push;
use zinc_build::Sub;

use crate::error::RuntimeError;
use crate::tests::TestRunner;
use crate::tests::TestingError;

#[test]
fn unsigned_positive_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(BigInt::from(255), IntegerType::U8.into()))
        .push(Push::new(BigInt::one(), IntegerType::U8.into()))
        .push(Add)
        .test(&[256]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_negative_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(BigInt::from(254), IntegerType::U8.into()))
        .push(Push::new(BigInt::from(255), IntegerType::U8.into()))
        .push(Sub)
        .test(&[-1]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_positive_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(BigInt::from(127), IntegerType::I8.into()))
        .push(Push::new(BigInt::one(), IntegerType::I8.into()))
        .push(Add)
        .test(&[128]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_negative_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(BigInt::from(-128), IntegerType::I8.into()))
        .push(Push::new(BigInt::one(), IntegerType::I8.into()))
        .push(Sub)
        .test(&[-129]);

    match res.err().expect(zinc_const::panic::TEST_DATA_VALID) {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_positive_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(BigInt::from(254), IntegerType::U8.into()))
        .push(Push::new(BigInt::one(), IntegerType::U8.into()))
        .push(Add)
        .test(&[255])
}

#[test]
fn unsigned_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(BigInt::from(255), IntegerType::U8.into()))
        .push(Push::new(BigInt::from(255), IntegerType::U8.into()))
        .push(Sub)
        .test(&[0])
}

#[test]
fn signed_positive_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(BigInt::from(126), IntegerType::I8.into()))
        .push(Push::new(BigInt::one(), IntegerType::I8.into()))
        .push(Add)
        .test(&[127])
}

#[test]
fn signed_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(BigInt::from(-127), IntegerType::I8.into()))
        .push(Push::new(BigInt::one(), IntegerType::I8.into()))
        .push(Sub)
        .test(&[-128])
}
