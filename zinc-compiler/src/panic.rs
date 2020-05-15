//!
//! The Zinc compiler panic index.
//!

// compiler phase responsibility
pub static VALIDATED_DURING_SOURCE_CODE_MAPPING: &str = "Validated during source code mapping";
pub static VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during lexical analysis";
pub static VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during syntax analysis";
pub static VALIDATED_DURING_SEMANTIC_ANALYSIS: &str = "Validated during semantic analysis";

// auxiliary
pub static MUTEX_SYNC: &str = "Mutexes never panic";
pub static JSON_TEMPLATE_SERIALIZATION: &str = "JSON serialization never panicks: ";
pub static BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";
pub static LOCATION_ALWAYS_EXISTS: &str = "Location always exists";

// test
#[allow(dead_code)]
pub static TEST_DATA: &str = "Test data is always valid";
