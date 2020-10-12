//!
//! The `mod` statement semantic analyzer.
//!

#[cfg(test)]
mod tests;

use zinc_syntax::Identifier;
use zinc_syntax::ModStatement;

use crate::semantic::error::Error;

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
