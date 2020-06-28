//!
//! The Zinc compiler panic index.
//!

// compiler phase responsibility
pub static VALIDATED_DURING_SOURCE_CODE_MAPPING: &str = "Validated during source code mapping";
pub static VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during lexical analysis";
pub static VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during syntax analysis";
pub static VALIDATED_DURING_SEMANTIC_ANALYSIS: &str = "Validated during semantic analysis";
pub static VALIDATED_DURING_BYTECODE_GENERATION: &str = "Validated during bytecode generation";

// auxiliary
pub static MUTEX_SYNC: &str = "Mutexes never panic";
pub static LAST_SHARED_REFERENCE: &str = "There are no other references at this point";
pub static JSON_TEMPLATE_SERIALIZATION: &str = "JSON serialization never panicks: ";
pub static BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";
pub static LOCATION_ALWAYS_EXISTS: &str = "Location always exists";

// test
pub static TEST_DATA_VALID: &str = "Test data is always valid";
