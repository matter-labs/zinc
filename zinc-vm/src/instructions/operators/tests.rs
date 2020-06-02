//!
//! The Zinc VM overflow tests.
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
        .add(Push::new(255.into(), IntegerType::U8.into()))
        .add(Push::new(1.into(), IntegerType::U8.into()))
        .add(Add)
        .test(&[256]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_negative_overflow_fail() {
    let res = TestRunner::new()
        .add(Push::new(254.into(), IntegerType::U8.into()))
        .add(Push::new(255.into(), IntegerType::U8.into()))
        .add(Sub)
        .test(&[-1]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_positive_overflow_fail() {
    let res = TestRunner::new()
        .add(Push::new(127.into(), IntegerType::I8.into()))
        .add(Push::new(1.into(), IntegerType::I8.into()))
        .add(Add)
        .test(&[128]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_negative_overflow_fail() {
    let res = TestRunner::new()
        .add(Push::new((-128).into(), IntegerType::I8.into()))
        .add(Push::new(1.into(), IntegerType::I8.into()))
        .add(Sub)
        .test(&[-129]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_positive_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .add(Push::new(254.into(), IntegerType::U8.into()))
        .add(Push::new(1.into(), IntegerType::U8.into()))
        .add(Add)
        .test(&[255])
}

#[test]
fn unsigned_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .add(Push::new(255.into(), IntegerType::U8.into()))
        .add(Push::new(255.into(), IntegerType::U8.into()))
        .add(Sub)
        .test(&[0])
}

#[test]
fn signed_positive_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .add(Push::new(126.into(), IntegerType::I8.into()))
        .add(Push::new(1.into(), IntegerType::I8.into()))
        .add(Add)
        .test(&[127])
}

#[test]
fn signed_negative_overflow_ok() -> Result<(), TestingError> {
    TestRunner::new()
        .add(Push::new((-127).into(), IntegerType::I8.into()))
        .add(Push::new(1.into(), IntegerType::I8.into()))
        .add(Sub)
        .test(&[-128])
}
