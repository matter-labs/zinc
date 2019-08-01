//!
//! The syntax analysis.
//!

mod analyzer;
mod error;
mod state;
mod r#type;
mod keyword;

pub use self::analyzer::Analyzer;
pub use self::error::Error;
pub use self::state::State;
pub use self::keyword::Keyword;
pub use self::r#type::Name as TypeName;
