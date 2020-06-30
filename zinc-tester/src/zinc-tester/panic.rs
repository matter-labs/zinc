//!
//! The Zinc tester panic index.
//!

/// The test directory contains some invalid tests. Causes a panic to show backtrace.
pub static TEST_DIRECTORY_INVALID: &str = "The test files directory must be valid";
/// The `rayon` thread pool initialization panic. Should always be successful.
pub static RAYON_POOL_INITIALIZATION: &str = "The thread pool is initialized only once";
