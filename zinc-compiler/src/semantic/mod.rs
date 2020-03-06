//!
//! The semantic analysis.
//!

mod analyzer;
mod bytecode;
mod caster;
mod element;
mod error;
mod scope;
mod tests;

pub use self::analyzer::entry_point::Analyzer as EntryPointAnalyzer;
pub use self::analyzer::module::Analyzer as ModuleAnalyzer;
pub use self::bytecode::Bytecode;
pub use self::error::Error;
pub use self::scope::Scope;

pub(crate) use self::caster::error::Error as CasterError;
pub(crate) use self::element::constant::error::Error as ConstantError;
pub(crate) use self::element::constant::integer::error::Error as IntegerConstantError;
pub(crate) use self::element::error::Error as ElementError;
pub(crate) use self::element::place::error::Error as PlaceError;
pub(crate) use self::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
pub(crate) use self::element::r#type::function::error::Error as FunctionError;
pub(crate) use self::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionError;
pub(crate) use self::element::value::array::error::Error as ArrayValueError;
pub(crate) use self::element::value::error::Error as ValueError;
pub(crate) use self::element::value::integer::error::Error as IntegerValueError;
pub(crate) use self::element::value::structure::error::Error as StructureValueError;
pub(crate) use self::element::value::tuple::error::Error as TupleValueError;
pub(crate) use self::scope::error::Error as ScopeError;

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
