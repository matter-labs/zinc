//!
//! Semantic tools library.
//!

mod casting;
mod inference;

pub use self::casting::validate as validate_casting;
pub use self::casting::Error as CastingError;
pub use self::inference::enough_bitlength as infer_enough_bitlength;
pub use self::inference::integer_literal as infer_integer_literal;
pub use self::inference::Error as InferenceError;

pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;

pub const MAX_VALUE_BYTE: usize = 256;

pub const BITLENGTH_BOOLEAN: usize = 1;
pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_MAX_INT: usize = 248;
pub const BITLENGTH_FIELD: usize = 254;
pub const BITLENGTH_FIELD_PADDED: usize = 256;
