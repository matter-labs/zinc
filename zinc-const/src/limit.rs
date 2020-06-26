//!
//! The Zinc limit constants.
//!

/// The `pedersen` hash maximal input size in bytes.
pub const PEDERSEN_HASH_INPUT_BYTES: usize = PEDERSEN_HASH_INPUT_BITS / crate::bitlength::BYTE;
/// The `pedersen` hash maximal input size in bits.
pub const PEDERSEN_HASH_INPUT_BITS: usize = 512;

/// The `schnorr` message maximal size in bytes.
pub const SCHNORR_MESSAGE_BYTES: usize = 31;
/// The `schnorr` message maximal size in bits.
pub const SCHNORR_MESSAGE_BITS: usize = SCHNORR_MESSAGE_BYTES * crate::bitlength::BYTE;
