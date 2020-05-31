//!
//! The VM state data stack tests.
//!

#![cfg(test)]

use num_bigint::BigInt;
use num_bigint::ToBigInt;

use franklin_crypto::circuit::test::TestConstraintSystem;
use pairing::bn256::Bn256;

use zinc_bytecode::ScalarType;

use crate::core::state::cell::Cell;
use crate::core::state::data_stack::DataStack;
use crate::gadgets::misc::Gadgets;
use crate::gadgets::scalar::Scalar;
use crate::Engine;

fn assert_cell_eq<E: Engine>(cell: Cell<E>, value: BigInt) {
    let Cell::Value(v) = cell;
    assert_eq!(v.to_bigint().unwrap(), value);
}

#[test]
fn test_get_set() {
    let mut ds = DataStack::new();
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let op = Gadgets::new(&mut cs);
    let value = op.constant_bigint(&42.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value)).unwrap();

    assert_cell_eq(ds.get(4).unwrap(), 42.into());
}

#[test]
fn test_fork_merge_true() {
    let mut ds = DataStack::new();
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let mut ops = Gadgets::new(&mut cs);
    let value = ops.constant_bigint(&42.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value)).unwrap();

    ds.fork();

    assert_cell_eq(ds.get(4).unwrap(), 42.into());

    let value2 = ops.constant_bigint(&13.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value2)).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 13.into());

    let condition = Scalar::new_constant_bool(true);
    ds.merge(condition, &mut ops).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 13.into());
}

#[test]
fn test_fork_merge_false() {
    let mut ds = DataStack::new();
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let mut ops = Gadgets::new(&mut cs);
    let value = ops.constant_bigint(&42.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value)).unwrap();

    ds.fork();

    assert_cell_eq(ds.get(4).unwrap(), 42.into());

    let value2 = ops.constant_bigint(&13.into(), ScalarType::Field).unwrap();
    ds.set(4, Cell::Value(value2)).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 13.into());

    let condition = Scalar::new_constant_bool(false);
    ds.merge(condition, &mut ops).unwrap();
    assert_cell_eq(ds.get(4).unwrap(), 42.into());
}
