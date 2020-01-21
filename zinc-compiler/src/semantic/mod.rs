//!
//! The semantic analysis.
//!

mod analyzer;
mod bytecode;
mod caster;
mod element;
mod scope;
mod tests;

pub use self::analyzer::BinaryAnalyzer;
pub use self::analyzer::Error;
pub use self::analyzer::ExpressionAnalyzer;
pub use self::analyzer::LibraryAnalyzer;
pub use self::analyzer::StatementAnalyzer;
pub use self::analyzer::TranslationHint;
pub use self::bytecode::Bytecode;
pub use self::caster::Caster;
pub use self::caster::Error as CasterError;
pub use self::element::Array;
pub use self::element::ArrayValueError;
pub use self::element::Constant;
pub use self::element::ConstantError;
pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::FieldAccessResult;
pub use self::element::FunctionBehavior;
pub use self::element::IndexAccessResult;
pub use self::element::IntegerConstant;
pub use self::element::IntegerConstantError;
pub use self::element::IntegerValue;
pub use self::element::IntegerValueError;
pub use self::element::Path;
pub use self::element::Place;
pub use self::element::PlaceError;
pub use self::element::Structure;
pub use self::element::StructureValueError;
pub use self::element::Tuple;
pub use self::element::TupleValueError;
pub use self::element::Type;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::scope::Error as ScopeError;
pub use self::scope::Item as ScopeItem;
pub use self::scope::Scope;
pub use self::scope::StaticItem as ScopeStaticItem;
pub use self::scope::VariableItem as ScopeVariableItem;

static PANIC_VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during the lexical analysis";
static PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during the syntax analysis";

static PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE: &str =
    "Scope stack balance is kept by the evaluation logic";
static PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND: &str =
    "Operand stack balance is kept by the evaluation logic";
static PANIC_THERE_MUST_ALWAYS_BE_A_CALL_STACK_POINTER: &str =
    "Call stack balance is kept by the evaluation logic";
static PANIC_THERE_MUST_ALWAYS_BE_THE_LAST_PATH_ELEMENT: &str =
    "Path last element existence is ensured by the evaluation logic";
static PANIC_THERE_MUST_ALWAYS_BE_A_SLICED_VALUE: &str =
    "Sliced value existence is ensured by the evaluation logic";

static PANIC_FUNCTION_ADDRESS_ALWAYS_EXISTS: &str =
    "Function address exists because the function type has been resolved above";
static PANIC_VALUE_CANNOT_BE_CREATED_FROM: &str = "Impossible to create a value from type: ";
static PANIC_SELF_ALIAS_DECLARATION: &str = "'Self' alias declaration is always valid";
static PANIC_PRETTY_SERIALIZATION_INVALID: &str = "Metadata JSON is always valid";
