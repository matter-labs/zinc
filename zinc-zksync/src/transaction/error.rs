//!
//! The transaction error.
//!

use failure::Fail;

use zksync_types::TokenId;

///
/// The transaction error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The transaction type in unsupported.
    #[fail(display = "the transaction type `{}` is not supported", _0)]
    UnsupportedTransaction(&'static str),
    /// The token ID is unknown.
    #[fail(display = "the token ID {} is not supported", _0)]
    UnsupportedToken(TokenId),
}
