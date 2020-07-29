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

/// The builder pattern entity must be provided with the specified value, unless it is not a bug.
pub static BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";

/// The source code mapping compiler phase responsibility.
pub static VALIDATED_DURING_SOURCE_CODE_MAPPING: &str = "Validated during source code mapping";

/// The lexical analysis compiler phase responsibility.
pub static VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during lexical analysis";

/// The syntax analysis compiler phase responsibility.
pub static VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during syntax analysis";

/// The semantic analysis compiler phase responsibility.
pub static VALIDATED_DURING_SEMANTIC_ANALYSIS: &str = "Validated during semantic analysis";

/// The target code generation compiler phase responsibility.
pub static VALIDATED_DURING_TARGET_CODE_GENERATION: &str =
    "Validated during target code generation";
