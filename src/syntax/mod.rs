//!
//! The syntax analysis.
//!

mod analyzer;
mod circuit;
mod error;
mod identificator;
mod input;
mod keyword;
mod r#type;
mod witness;

pub use self::analyzer::Analyzer;
pub use self::circuit::CircuitProgram;
pub use self::error::Error;
pub use self::identificator::Error as IdentificatorError;
pub use self::identificator::Identificator;
pub use self::input::Input;
pub use self::keyword::Keyword;
pub use self::r#type::Keyword as TypeKeyword;
pub use self::r#type::KeywordError as TypeKeywordError;
pub use self::r#type::Type;
pub use self::witness::Witness;

pub const SEMICOLON: char = ';';
pub const COLON: char = ':';
pub const GROUP_DELIMITER: proc_macro2::Delimiter = proc_macro2::Delimiter::Brace;
