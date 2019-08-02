//!
//! The syntax analysis.
//!

mod analyzer;
mod error;
mod r#type;
mod keyword;
mod variable;

pub use self::analyzer::Analyzer;
pub use self::error::Error;
pub use self::keyword::Keyword;
pub use self::variable::Name as VariableName;
pub use self::variable::NameError as VariableNameError;
pub use self::r#type::Name as TypeName;

pub const SEMICOLON: char = ';';
pub const COLON: char = ':';
pub const GROUP_DELIMITER: proc_macro::Delimiter = proc_macro::Delimiter::Brace;
