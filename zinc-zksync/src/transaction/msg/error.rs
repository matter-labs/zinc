//!
//! The transfer error.
//!

use failure::Fail;

///
/// The transfer error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The transfer argument is missing.
    #[fail(display = "the `{}` argument is missing", _0)]
    ArgumentMissing(&'static str),
    /// The transfer argument is missing.
    #[fail(
        display = "the `{}` argument is malformed, expected a single structure or array",
        _0
    )]
    ArgumentInvalidFormat(&'static str),
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
    /// The token ID is invalid.
    #[fail(display = "token ID is invalid: {} (expected a decimal number)", _0)]
    TokenIdInvalid(::std::num::ParseIntError),
    /// The amount is invalid.
    #[fail(display = "amount is invalid: {} (expected a decimal number)", _0)]
    AmountInvalid(zinc_math::BigIntError),
}
