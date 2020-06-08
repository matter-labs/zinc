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
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

fn assert_cell_eq<E: IEngine>(cell: Cell<E>, value: BigInt) {
    let Cell::Value(v) = cell;
    assert_eq!(v.to_bigint().unwrap(), value);
}

#[test]
fn test_get_set() {
    let mut ds = DataStack::<Bn256>::new();
    let value =
        gadgets::scalar::fr_bigint::bigint_to_fr_scalar(&42.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value)).unwrap();

    assert_cell_eq(ds.get(4).unwrap(), 42.into());
}

#[test]
fn test_fork_merge_true() {
    let mut ds = DataStack::new();
    let cs = TestConstraintSystem::<Bn256>::new();
    let value =
        gadgets::scalar::fr_bigint::bigint_to_fr_scalar(&42.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value)).unwrap();

    ds.fork();

    assert_cell_eq(ds.get(4).unwrap(), 42.into());

    let value2 =
        gadgets::scalar::fr_bigint::bigint_to_fr_scalar(&13.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value2)).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 13.into());

    let condition = Scalar::new_constant_bool(true);
    ds.merge(cs, condition).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 13.into());
}

#[test]
fn test_fork_merge_false() {
    let mut ds = DataStack::new();
    let cs = TestConstraintSystem::<Bn256>::new();
    let value =
        gadgets::scalar::fr_bigint::bigint_to_fr_scalar(&42.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value)).unwrap();

    ds.fork();

    assert_cell_eq(ds.get(4).unwrap(), 42.into());

    let value2 =
        gadgets::scalar::fr_bigint::bigint_to_fr_scalar(&13.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value2)).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 13.into());

    let condition = Scalar::new_constant_bool(false);
    ds.merge(cs, condition).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 42.into());
}
