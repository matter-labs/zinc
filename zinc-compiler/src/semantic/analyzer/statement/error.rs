//!
//! The semantic analyzer statement error.
//!

use crate::semantic::analyzer::statement::r#for::error::Error as ForStatementError;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::analyzer::statement::r#use::error::Error as UseStatementError;

///
/// The semantic analyzer statement error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The `for` statement analysis error.
    For(ForStatementError),
    /// The `impl` statement analysis error.
    Impl(ImplStatementError),
    /// The `use` statement analysis error.
    Use(UseStatementError),
}
