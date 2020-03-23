use crate::instructions::testing_utils::{TestingError, VMTestRunner};
use crate::RuntimeError;
use zinc_bytecode::instructions::*;
use zinc_bytecode::scalar::IntegerType;

#[test]
fn unsigned_positive_overflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new(255.into(), IntegerType::U8.into()))
        .add(PushConst::new(1.into(), IntegerType::U8.into()))
        .add(Add)
        .test(&[256]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_negative_overflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new(254.into(), IntegerType::U8.into()))
        .add(PushConst::new(255.into(), IntegerType::U8.into()))
        .add(Sub)
        .test(&[-1]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_positive_overflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new(127.into(), IntegerType::I8.into()))
        .add(PushConst::new(1.into(), IntegerType::I8.into()))
        .add(Add)
        .test(&[128]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_negative_overflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new((-128).into(), IntegerType::I8.into()))
        .add(PushConst::new(1.into(), IntegerType::I8.into()))
        .add(Sub)
        .test(&[-129]);

    match res.err().expect("expected overflow error") {
        TestingError::RuntimeError(RuntimeError::ValueOverflow { .. }) => {}
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_positive_overflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new(254.into(), IntegerType::U8.into()))
        .add(PushConst::new(1.into(), IntegerType::U8.into()))
        .add(Add)
        .test(&[255])
}

#[test]
fn unsigned_negative_overflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new(255.into(), IntegerType::U8.into()))
        .add(PushConst::new(255.into(), IntegerType::U8.into()))
        .add(Sub)
        .test(&[0])
}

#[test]
fn signed_positive_overflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new(126.into(), IntegerType::I8.into()))
        .add(PushConst::new(1.into(), IntegerType::I8.into()))
        .add(Add)
        .test(&[127])
}

#[test]
fn signed_negative_overflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new((-127).into(), IntegerType::I8.into()))
        .add(PushConst::new(1.into(), IntegerType::I8.into()))
        .add(Sub)
        .test(&[-128])
}
