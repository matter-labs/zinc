//!
//! The semantic analysis.
//!

pub mod analyzer;
pub mod bytecode;
pub mod caster;
pub mod element;
pub mod scope;
pub mod tests;

pub use self::analyzer::binary::Analyzer as BinaryAnalyzer;
pub use self::analyzer::error::Error;
pub use self::analyzer::library::Analyzer as LibraryAnalyzer;
pub use self::bytecode::Bytecode;
pub use self::scope::Scope;

static PANIC_VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during the lexical analysis";
static PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during the syntax analysis";

static PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE: &str =
    "Scope stack balance is kept by the evaluation logic";
static PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND: &str =
    "Operand stack balance is kept by the evaluation logic";
static PANIC_THERE_MUST_ALWAYS_BE_A_CALL_STACK_POINTER: &str =
    "Call stack balance is kept by the evaluation logic";

static PANIC_FUNCTION_ADDRESS_ALWAYS_EXISTS: &str =
    "Function address exists because the function type has been resolved above";
static PANIC_JSON_TEMPLATE_SERIALIZATION: &str =
    "JSON templates serialization must be always successful: ";
