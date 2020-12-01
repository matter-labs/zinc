//!
//! The VM state data stack tests.
//!

use num::bigint::ToBigInt;
use num::BigInt;

use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

use crate::core::execution_state::cell::Cell;
use crate::core::execution_state::data_stack::DataStack;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

fn assert_cell_eq<E: IEngine>(cell: Cell<E>, value: BigInt) {
    let Cell::Value(v) = cell;
    assert_eq!(
        v.to_bigint().expect(zinc_const::panic::TEST_DATA_VALID),
        value
    );
}

#[test]
fn test_get_set() {
    let mut ds = DataStack::<Bn256>::new();
    let value = Scalar::new_constant_usize(42, zinc_types::ScalarType::Field);
    ds.set(4, Cell::Value(value))
        .expect(zinc_const::panic::TEST_DATA_VALID);

    assert_cell_eq(
        ds.get(4).expect(zinc_const::panic::TEST_DATA_VALID),
        BigInt::from(42),
    );
}

#[test]
fn test_fork_merge_true() {
    let mut ds = DataStack::new();
    let cs = TestConstraintSystem::<Bn256>::new();
    let value = Scalar::new_constant_usize(42, zinc_types::ScalarType::Field);
    ds.set(4, Cell::Value(value))
        .expect(zinc_const::panic::TEST_DATA_VALID);

    ds.fork();

    assert_cell_eq(
        ds.get(4).expect(zinc_const::panic::TEST_DATA_VALID),
        BigInt::from(42),
    );

    let value2 = Scalar::new_constant_usize(13, zinc_types::ScalarType::Field);
    ds.set(4, Cell::Value(value2))
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_cell_eq(
        ds.get(4).expect(zinc_const::panic::TEST_DATA_VALID),
        BigInt::from(13),
    );

    let condition = Scalar::new_constant_bool(true);
    ds.merge(cs, condition)
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_cell_eq(
        ds.get(4).expect(zinc_const::panic::TEST_DATA_VALID),
        BigInt::from(13),
    );
}

#[test]
fn test_fork_merge_false() {
    let mut ds = DataStack::new();
    let cs = TestConstraintSystem::<Bn256>::new();
    let value = Scalar::new_constant_usize(42, zinc_types::ScalarType::Field);
    ds.set(4, Cell::Value(value))
        .expect(zinc_const::panic::TEST_DATA_VALID);

    ds.fork();

    assert_cell_eq(
        ds.get(4).expect(zinc_const::panic::TEST_DATA_VALID),
        BigInt::from(42),
    );

    let value2 = Scalar::new_constant_usize(13, zinc_types::ScalarType::Field);
    ds.set(4, Cell::Value(value2))
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_cell_eq(
        ds.get(4).expect(zinc_const::panic::TEST_DATA_VALID),
        BigInt::from(13),
    );

    let condition = Scalar::new_constant_bool(false);
    ds.merge(cs, condition)
        .expect(zinc_const::panic::TEST_DATA_VALID);
    assert_cell_eq(
        ds.get(4).expect(zinc_const::panic::TEST_DATA_VALID),
        BigInt::from(42),
    );
}
