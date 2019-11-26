//!
//! The semantic analysis.
//!

#![allow(clippy::large_enum_variant)]

mod analyzer;
mod casting;
mod element;
mod error;
mod inference;
mod scope;
mod tests;

pub use self::analyzer::Analyzer;
pub use self::casting::validate as validate_casting;
pub use self::casting::Error as CastingError;
pub use self::element::Boolean;
pub use self::element::Element;
pub use self::element::Error as ElementError;
pub use self::element::Integer;
pub use self::element::IntegerError;
pub use self::element::Place;
pub use self::element::PlaceDescriptor;
pub use self::element::PlaceError;
pub use self::element::Value;
pub use self::element::ValueError;
pub use self::error::Error;
pub use self::inference::integer_literal as infer_integer_literal;
pub use self::inference::Error as InferenceError;
pub use self::scope::Assignment as ScopeAssignment;
pub use self::scope::Error as ScopeError;
pub use self::scope::Item as ScopeItem;
pub use self::scope::Scope;

static PANIC_RESOLUTION_FUNCTION_MAIN: &str = "Presence of the 'main' function is checked above";
static PANIC_VALIDATED_DURING_LEXICAL_ANALYSIS: &str =
    "Integer literals are validated during the lexical analysis";
