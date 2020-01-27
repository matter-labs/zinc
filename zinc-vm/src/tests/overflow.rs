use crate::instructions::testing_utils::{VMTestRunner, TestingError};
use zinc_bytecode::instructions::*;

#[test]
fn unsigned_overflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new(255.into(), false, 8))
        .add(PushConst::new(1.into(), false, 8))
        .add(Add)
        .test(&[256]);

    match res.err().expect("expected overflow error") {
        TestingError::Unsatisfied => {},
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_underflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new(254.into(), false, 8))
        .add(PushConst::new(255.into(), false, 8))
        .add(Sub)
        .test(&[-1]);

    match res.err().expect("expected overflow error") {
        TestingError::Unsatisfied => {},
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_overflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new(127.into(), true, 8))
        .add(PushConst::new(1.into(), true, 8))
        .add(Add)
        .test(&[128]);

    match res.err().expect("expected overflow error") {
        TestingError::Unsatisfied => {},
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn signed_underflow_fail() {
    let res = VMTestRunner::new()
        .add(PushConst::new((-128).into(), true, 8))
        .add(PushConst::new(1.into(), true, 8))
        .add(Sub)
        .test(&[-129]);

    match res.err().expect("expected overflow error") {
        TestingError::Unsatisfied => {},
        err => panic!("expected overflow error, got {:?} instead", err),
    }
}

#[test]
fn unsigned_overflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new(254.into(), false, 8))
        .add(PushConst::new(1.into(), false, 8))
        .add(Add)
        .test(&[255])
}

#[test]
fn unsigned_underflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new(255.into(), false, 8))
        .add(PushConst::new(255.into(), false, 8))
        .add(Sub)
        .test(&[0])
}

#[test]
fn signed_overflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new(126.into(), true, 8))
        .add(PushConst::new(1.into(), true, 8))
        .add(Add)
        .test(&[127])
}

#[test]
fn signed_underflow_ok() -> Result<(), TestingError> {
    VMTestRunner::new()
        .add(PushConst::new((-127).into(), true, 8))
        .add(PushConst::new(1.into(), true, 8))
        .add(Sub)
        .test(&[-128])
}
