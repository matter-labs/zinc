//!
//! The transaction error.
//!

use failure::Fail;

///
/// The transaction error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The transaction type in unsupported.
    #[fail(display = "the transaction type `{}` in unsupported", _0)]
    UnsupportedTransaction(&'static str),
}
