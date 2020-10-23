//!
//! The miscellaneous math functions tests.
//!

use crate::misc::floor_to_power_of_two;
use crate::misc::log2ceil;

#[test]
fn ok_log2ceil() {
    assert_eq!(log2ceil(0), 0);
    assert_eq!(log2ceil(1), 0);
    assert_eq!(log2ceil(2), 1);
    assert_eq!(log2ceil(3), 2);
    assert_eq!(log2ceil(4), 2);
    assert_eq!(log2ceil(5), 3);
    assert_eq!(log2ceil(8), 3);
    assert_eq!(log2ceil(9), 4);
    assert_eq!(log2ceil(1024), 10);
    assert_eq!(log2ceil(1025), 11);
}

#[test]
fn ok_floor_to_power_of_two() {
    assert_eq!(floor_to_power_of_two(1), 1);
    assert_eq!(floor_to_power_of_two(2), 2);
    assert_eq!(floor_to_power_of_two(3), 2);
    assert_eq!(floor_to_power_of_two(4), 4);
    assert_eq!(floor_to_power_of_two(5), 4);
    assert_eq!(floor_to_power_of_two(7), 4);
    assert_eq!(floor_to_power_of_two(8), 8);
}
