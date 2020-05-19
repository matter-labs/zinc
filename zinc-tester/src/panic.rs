//!
//! The Zinc tester panic index.
//!

pub static MUTEX_SYNC: &str = "Mutexes never panic";
pub static LAST_SHARED_REFERENCE: &str = "There are no other references at this point";

pub static TEST_DIRECTORY_INVALID: &str = "The test files directory must be valid";
pub static MAIN_ENTRY_ID: &str = "The 'main' entry always exists";
