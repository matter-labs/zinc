//!
//! The `mod` statement semantic analyzer.
//!

#[cfg(test)]
mod tests;

use crate::semantic::error::Error;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::module::Statement as ModStatement;

///
/// The `mod` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only module declaration statement.
    ///
    /// Is not used for now.
    ///
    pub fn analyze(statement: ModStatement) -> Result<Identifier, Error> {
        Ok(statement.identifier)
    }
}
