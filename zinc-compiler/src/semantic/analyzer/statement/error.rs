//!
//! The semantic analyzer statement error.
//!

use crate::semantic::analyzer::statement::r#for::error::Error as ForStatementError;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::analyzer::statement::r#use::error::Error as UseStatementError;

#[derive(Debug, PartialEq)]
pub enum Error {
    For(ForStatementError),
    Impl(ImplStatementError),
    Use(UseStatementError),
}
