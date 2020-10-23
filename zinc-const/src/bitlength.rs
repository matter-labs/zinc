//!
//! The Zinc integer bitlength constants.
//!

/// The `bool` type bitlength.
pub const BOOLEAN: usize = 1;

/// The `u8` type or byte bitlength.
pub const BYTE: usize = 8;

/// The index type (usually `u64`) bitlength.
pub const INDEX: usize = 64;

/// The `u248` or `i248` types bitlength.
pub const INTEGER_MAX: usize = 248;

/// The `field` type bitlength.
pub const FIELD: usize = 254;

/// The `field` type padded to a multiple of 8 bitlength.
pub const FIELD_PADDED: usize = FIELD + (BYTE - FIELD % BYTE);

/// The `sha256` hash bitlength.
pub const SHA256_HASH: usize = crate::size::SHA256_HASH * BYTE;

/// The zkSync token ID bitlength.
pub const TOKEN_ID: usize = BYTE * 2;

/// The smart contract balance bitlength.
pub const BALANCE: usize = self::INTEGER_MAX;

/// The ETH address bitlength.
pub const ETH_ADDRESS: usize = crate::size::ETH_ADDRESS * BYTE;
