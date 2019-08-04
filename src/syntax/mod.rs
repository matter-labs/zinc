//!
//! The syntax analysis.
//!

mod analyzer;
mod circuit;
mod error;
mod identifier;
mod input;
mod keyword;
mod r#type;
mod witness;

pub use self::analyzer::Analyzer;
pub use self::analyzer::TypeAnalyzer;
pub use self::circuit::CircuitProgram;
pub use self::error::Error;
pub use self::identifier::Error as IdentifierError;
pub use self::identifier::Identifier;
pub use self::input::Builder as InputBuilder;
pub use self::input::Input;
pub use self::keyword::Keyword;
pub use self::r#type::Builder as TypeBuilder;
pub use self::r#type::BuilderError as TypeBuilderError;
pub use self::r#type::Keyword as TypeKeyword;
pub use self::r#type::KeywordError as TypeKeywordError;
pub use self::r#type::Type;
pub use self::witness::Builder as WitnessBuilder;
pub use self::witness::Witness;

pub const SEMICOLON: char = ';';
pub const COLON: char = ':';
pub const GROUP_DELIMITER: proc_macro2::Delimiter = proc_macro2::Delimiter::Brace;
