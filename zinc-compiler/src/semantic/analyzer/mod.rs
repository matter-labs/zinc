//!
//! The semantic analyzer.
//!

mod binary;
mod error;
mod expression;
mod library;
mod statement;
mod translation_hint;

pub use self::binary::Analyzer as BinaryAnalyzer;
pub use self::error::Error;
pub use self::expression::Analyzer as ExpressionAnalyzer;
pub use self::library::Analyzer as LibraryAnalyzer;
pub use self::statement::Analyzer as StatementAnalyzer;
pub use self::translation_hint::TranslationHint;
