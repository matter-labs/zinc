//!
//! The VM state data stack tests.
//!

#![cfg(test)]

use num_bigint::BigInt;
use num_bigint::ToBigInt;

use franklin_crypto::circuit::test::TestConstraintSystem;
use pairing::bn256::Bn256;

use zinc_bytecode::ScalarType;

use crate::core::execution_state::cell::Cell;
use crate::core::execution_state::data_stack::DataStack;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

fn assert_cell_eq<E: IEngine>(cell: Cell<E>, value: BigInt) {
    let Cell::Value(v) = cell;
    assert_eq!(v.to_bigint().expect(crate::panic::TEST_DATA_VALID), value);
}

#[test]
fn test_get_set() {
    let mut ds = DataStack::<Bn256>::new();
    let value = Scalar::new_constant_bigint(&42.into(), ScalarType::Field)
        .expect(crate::panic::TEST_DATA_VALID);
    ds.set(4, Cell::Value(value))
        .expect(crate::panic::TEST_DATA_VALID);

    assert_cell_eq(ds.get(4).expect(crate::panic::TEST_DATA_VALID), 42.into());
}

#[test]
fn test_fork_merge_true() {
    let mut ds = DataStack::new();
    let cs = TestConstraintSystem::<Bn256>::new();
    let value = Scalar::new_constant_bigint(&42.into(), ScalarType::Field)
        .expect(crate::panic::TEST_DATA_VALID);
    ds.set(4, Cell::Value(value))
        .expect(crate::panic::TEST_DATA_VALID);

    ds.fork();

    assert_cell_eq(ds.get(4).expect(crate::panic::TEST_DATA_VALID), 42.into());

    let value2 = Scalar::new_constant_bigint(&13.into(), ScalarType::Field)
        .expect(crate::panic::TEST_DATA_VALID);
    ds.set(4, Cell::Value(value2))
        .expect(crate::panic::TEST_DATA_VALID);
    assert_cell_eq(ds.get(4).expect(crate::panic::TEST_DATA_VALID), 13.into());

    let condition = Scalar::new_constant_bool(true);
    ds.merge(cs, condition)
        .expect(crate::panic::TEST_DATA_VALID);
    assert_cell_eq(ds.get(4).expect(crate::panic::TEST_DATA_VALID), 13.into());
}

#[test]
fn test_fork_merge_false() {
    let mut ds = DataStack::new();
    let cs = TestConstraintSystem::<Bn256>::new();
    let value = Scalar::new_constant_bigint(&42.into(), ScalarType::Field)
        .expect(crate::panic::TEST_DATA_VALID);
    ds.set(4, Cell::Value(value))
        .expect(crate::panic::TEST_DATA_VALID);

    ds.fork();

    assert_cell_eq(ds.get(4).expect(crate::panic::TEST_DATA_VALID), 42.into());

    let value2 = Scalar::new_constant_bigint(&13.into(), ScalarType::Field)
        .expect(crate::panic::TEST_DATA_VALID);
    ds.set(4, Cell::Value(value2))
        .expect(crate::panic::TEST_DATA_VALID);
    assert_cell_eq(ds.get(4).expect(crate::panic::TEST_DATA_VALID), 13.into());

    let condition = Scalar::new_constant_bool(false);
    ds.merge(cs, condition)
        .expect(crate::panic::TEST_DATA_VALID);
    assert_cell_eq(ds.get(4).expect(crate::panic::TEST_DATA_VALID), 42.into());
}
