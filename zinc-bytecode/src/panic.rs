//!
//! The Zinc bytecode panic index.
//!

/// The metadata consists of `BigInt`s at the lowest level, thus is always successfully serialized.
pub static BINARY_SERIALIZATION: &str = "Binary serialization never panicks: ";
