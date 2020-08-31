//!
//! The Zinc common utilities.
//!

pub(crate) mod logger;
pub(crate) mod math;

pub use crate::logger::initialize as initialize_logger;
pub use crate::math::euclidean::div_rem as euclidean_div_rem;
pub use crate::math::floor_to_power_of_two;
pub use crate::math::inference::error::Error as InferenceError;
pub use crate::math::inference::literal_types as infer_literal_types;
pub use crate::math::inference::minimal_bitlength as infer_minimal_bitlength;
pub use crate::math::inference::r#type::Type as InferredType;
pub use crate::math::inference::result::Binary as BinaryInferenceResult;
pub use crate::math::log2ceil;
