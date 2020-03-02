mod bigint;
mod fr_bigint;

pub use bigint::*;
pub use fr_bigint::*;

pub mod math {
    pub fn log2ceil(n: usize) -> usize {
        let mut log = 0;
        while n > 1 << log {
            log += 1;
        }
        log
    }

    pub fn floor_to_power_of_two(n: usize) -> usize {
        let mut pow = 0;
        while n / 2 >= 1 << pow {
            pow += 1;
        }
        1 << pow
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_log2ceil() {
            assert_eq!(log2ceil(0), 0, "log2ceil(0)");
            assert_eq!(log2ceil(1), 0, "log2ceil(1)");
            assert_eq!(log2ceil(2), 1, "log2ceil(2)");
            assert_eq!(log2ceil(3), 2, "log2ceil(3)");
            assert_eq!(log2ceil(4), 2, "log2ceil(4)");
            assert_eq!(log2ceil(5), 3, "log2ceil(5)");
            assert_eq!(log2ceil(8), 3, "log2ceil(8)");
            assert_eq!(log2ceil(9), 4, "log2ceil(9)");
            assert_eq!(log2ceil(1024), 10, "log2ceil(1024)");
            assert_eq!(log2ceil(1025), 11, "log2ceil(1025)");
        }

        #[test]
        fn test_floor() {
            assert_eq!(floor_to_power_of_two(1), 1, "floor_to_power_of_two(1)");
            assert_eq!(floor_to_power_of_two(2), 2, "floor_to_power_of_two(2)");
            assert_eq!(floor_to_power_of_two(3), 2, "floor_to_power_of_two(3)");
            assert_eq!(floor_to_power_of_two(4), 4, "floor_to_power_of_two(4)");
            assert_eq!(floor_to_power_of_two(5), 4, "floor_to_power_of_two(5)");
            assert_eq!(floor_to_power_of_two(7), 4, "floor_to_power_of_two(7)");
            assert_eq!(floor_to_power_of_two(8), 8, "floor_to_power_of_two(8)");
        }
    }
}
