//!
//! The operator instructions overflow tests.
//!

#![cfg(test)]

use zinc_bytecode::Add;
use zinc_bytecode::IntegerType;
use zinc_bytecode::Push;
use zinc_bytecode::Sub;

use crate::error::RuntimeError;
use crate::tests::TestRunner;
use crate::tests::TestingError;

#[test]
fn unsigned_positive_overflow_fail() {
    let res = TestRunner::new()
        .push(Push::new(255.into(), IntegerType::U8.into()))
        .push(Push::new(1.into(), IntegerType::U8.into()))
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
        .push(Push::new(254.into(), IntegerType::U8.into()))
        .push(Push::new(255.into(), IntegerType::U8.into()))
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
        .push(Push::new(127.into(), IntegerType::I8.into()))
        .push(Push::new(1.into(), IntegerType::I8.into()))
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
        .push(Push::new((-128).into(), IntegerType::I8.into()))
        .push(Push::new(1.into(), IntegerType::I8.into()))
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
        .push(Push::new(254.into(), IntegerType::U8.into()))
        .push(Push::new(1.into(), IntegerType::U8.into()))
        .push(Add)
        .test(&[255])
}

#[test]
fn unsigned_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(255.into(), IntegerType::U8.into()))
        .push(Push::new(255.into(), IntegerType::U8.into()))
        .push(Sub)
        .test(&[0])
}

#[test]
fn signed_positive_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new(126.into(), IntegerType::I8.into()))
        .push(Push::new(1.into(), IntegerType::I8.into()))
        .push(Add)
        .test(&[127])
}

#[test]
fn signed_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .push(Push::new((-127).into(), IntegerType::I8.into()))
        .push(Push::new(1.into(), IntegerType::I8.into()))
        .push(Sub)
        .test(&[-128])
}
