//!
//! The Zinc math utilities.
//!

pub(crate) mod bigint;
pub(crate) mod error;
pub(crate) mod euclidean;
pub(crate) mod inference;
pub(crate) mod misc;

pub use crate::bigint::from_str as bigint_from_str;
pub use crate::error::Error;
pub use crate::euclidean::div_rem as euclidean_div_rem;
pub use crate::inference::literal_types as infer_literal_types;
pub use crate::inference::minimal_bitlength as infer_minimal_bitlength;
pub use crate::inference::r#type::Type as InferredType;
pub use crate::inference::result::Binary as BinaryInferenceResult;
pub use crate::misc::floor_to_power_of_two;
pub use crate::misc::log2ceil;

pub(crate) type Result<T> = std::result::Result<T, Error>;
