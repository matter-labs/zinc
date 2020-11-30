//!
//! The Zinc limit constants.
//!

/// The `pedersen` hash maximal input size in bytes.
pub const PEDERSEN_HASH_INPUT_BYTES: usize = 64;

/// The `pedersen` hash maximal input size in bits.
pub const PEDERSEN_HASH_INPUT_BITS: usize = PEDERSEN_HASH_INPUT_BYTES * crate::bitlength::BYTE;

/// The `schnorr` message maximal size in bytes.
pub const SCHNORR_MESSAGE_BYTES: usize = 31;

/// The `schnorr` message maximal size in bits.
pub const SCHNORR_MESSAGE_BITS: usize = SCHNORR_MESSAGE_BYTES * crate::bitlength::BYTE;

/// The Zinc compiler inner thread stack size.
pub const COMPILER_STACK_SIZE: usize = 64 * 1024 * 1024;

/// The JSON payload limit to fit large contract source code.
pub static JSON_PAYLOAD: usize = 16 * 1024 * 1024;
