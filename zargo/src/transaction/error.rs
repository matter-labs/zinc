//!
//! The transaction error.
//!

use failure::Fail;

///
/// The transaction error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// A required transaction field is missing.
    #[fail(display = "`{}` field is missing", _0)]
    FieldMissing(&'static str),
    /// A field is not a string.
    #[fail(display = "`{}` field must be a string", _0)]
    NotAString(&'static str),
    /// The sender address is invalid.
    #[fail(
        display = "sender address is invalid: {} (expected `0x[0-9A-Fa-f]{{40}}`)",
        _0
    )]
    SenderAddressInvalid(rustc_hex::FromHexError),
    /// The recipient address is invalid.
    #[fail(
        display = "recipient address is invalid: {} (expected `0x[0-9A-Fa-f]{{40}}`)",
        _0
    )]
    RecipientAddressInvalid(rustc_hex::FromHexError),
    /// The transaction token is invalid.
    #[fail(display = "token ID is invalid")]
    TokenIdInvalid,
    /// The transaction amount is invalid.
    #[fail(display = "amount is invalid: {} (expected a decimal number)", _0)]
    AmountInvalid(num_old::bigint::ParseBigIntError),
    /// The sender private key is invalid.
    #[fail(display = "sender private key is invalid: {}", _0)]
    SenderPrivateKeyInvalid(rustc_hex::FromHexError),
    /// The sender address cannot be derived from the private key.
    #[fail(
        display = "could not derive the ETH address from the private key: {}",
        _0
    )]
    SenderAddressDeriving(failure::Error),
    /// The sender address does not match the private key.
    #[fail(display = "sender address does not match the private key")]
    SenderAddressPrivateKeyMismatch,
    /// The wallet initialization error.
    #[fail(display = "wallet initialization error: {}", _0)]
    WalletInitialization(zksync::error::ClientError),
    /// The account info retrieving error.
    #[fail(display = "account info retrieving error: {}", _0)]
    AccountInfoRetrieving(zksync::error::ClientError),
    /// The transaction signing error.
    #[fail(display = "signing error: {}", _0)]
    TransactionSigning(zksync::error::SignerError),
}
