//!
//! The semantic analysis.
//!

mod analyzer;
mod casting;
mod element;
mod error;
mod inference;
mod scope;
mod tests;

pub use self::analyzer::Analyzer;
pub use self::casting::cast as validate_casting;
pub use self::casting::Error as CastingError;
pub use self::element::Array;
pub use self::element::ArrayError;
pub use self::element::Constant;
pub use self::element::ConstantError;
pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::Integer;
pub use self::element::IntegerError;
pub use self::element::Place;
pub use self::element::PlaceDescriptor;
pub use self::element::PlaceError;
pub use self::element::Structure;
pub use self::element::StructureError;
pub use self::element::Tuple;
pub use self::element::Type;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::error::Error;
pub use self::inference::integer_literal as infer_integer_literal;
pub use self::inference::Error as InferenceError;
pub use self::scope::Error as ScopeError;
pub use self::scope::Item as ScopeItem;
pub use self::scope::Scope;

static PANIC_VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during the lexical analysis";
static PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during the syntax analysis";
static PANIC_VALIDATED_USING_CASTING_MODULE: &str =
    "Ensured to be an integer type in the casting module";
static PANIC_INSTRUCTION_FUNCTION_DECLARATION: &str =
    "Instruction functions are declared without errors";
static PANIC_RESOLUTION_FUNCTION_MAIN: &str = "Presence of the 'main' function is checked above";
static PANIC_RANGE_OPERATORS_ARE_UNAVAILABLE: &str = "Range operators are unavailable yet";
static PANIC_ALWAYS_EVALUATED: &str = "The element is always evaluated";
static PANIC_VALUE_CANNOT_BE_CREATED_FROM: &str = "Impossible to create a value from type: ";
static PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE: &str =
    "Scope stack balance is ensured by the evaluation logic";
static PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND: &str =
    "Operand stack balance is ensured by the evaluation logic";
