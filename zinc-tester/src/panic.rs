//!
//! The Zinc tester panic index.
//!

/// The mutex locking panic message.
pub static MUTEX_SYNC: &str = "Mutexes never panic";
/// The shared reference unwrapping panic message.
pub static LAST_SHARED_REFERENCE: &str = "There are no other references at this point";
