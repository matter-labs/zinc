//!
//! The Zinc compiler panic index.
//!

// compiler phase responsibility
pub static VALIDATED_DURING_SOURCE_CODE_MAPPING: &str = "Validated during source code mapping";
pub static VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during lexical analysis";
pub static VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during syntax analysis";
pub static VALIDATED_DURING_SEMANTIC_ANALYSIS: &str = "Validated during semantic analysis";

// syntax builder
pub static BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";
pub static BUILDER_TYPE_INVALID_KEYWORD: &str =
    "The type builder has got an unexpected non-type keyword: ";
pub static VALIDATED_BY_THE_TYPE_PARSER: &str =
    "Unreachable as long as the type parser works correctly";

// semantic stack balance
pub static THERE_MUST_ALWAYS_BE_AN_OPERAND: &str =
    "Operand stack balance is kept by the evaluation logic";
pub static THERE_MUST_ALWAYS_BE_A_SCOPE: &str =
    "Scope stack balance is kept by the evaluation logic";

// auxiliary
pub static LAST_SHARED_REFERENCE: &str = "There are no other references at this point";
pub static MUTEX_SYNC: &str = "Mutexes never panic";
pub static JSON_TEMPLATE_SERIALIZATION: &str = "JSON serialization never panicks: ";
pub static ENSURED_WHILE_RETURNING_ENTRIES: &str = "Ensured while returning the entries";
pub static LOCATION_ALWAYS_EXISTS: &str = "Location always exists";

// test
#[allow(dead_code)]
pub static TEST_DATA: &str = "Test data is always valid";
