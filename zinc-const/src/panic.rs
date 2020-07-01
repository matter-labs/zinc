//!
//! The Zinc panic constant messages.
//!

/// The mutex locking panic message.
pub static MUTEX_SYNC: &str = "Mutexes never panic";

/// The serialization is always valid, as all the types are known at compile-time.
pub static DATA_SERIALIZATION: &str = "JSON serialization never panicks: ";

/// The shared reference unwrapping panic message.
pub static LAST_SHARED_REFERENCE: &str = "There are no other references at this point";

/// The unit test data validity is checked by the test authors.
pub static TEST_DATA_VALID: &str = "Test data is always valid";

/// The `Result` or `Option` value is always set. Should be eliminated where possible.
pub static VALUE_ALWAYS_EXISTS: &str = "Value always exists";
