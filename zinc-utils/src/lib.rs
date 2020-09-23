//!
//! The Zinc common utilities.
//!

pub(crate) mod array;
pub(crate) mod bigint;
pub(crate) mod logger;
pub(crate) mod math;
pub(crate) mod token;

pub use crate::array::eth_address_from_vec;
pub use crate::array::eth_private_key_from_vec;
pub use crate::bigint::from_str_radix as bigint_from_str_radix;
pub use crate::logger::initialize as initialize_logger;
pub use crate::math::euclidean::div_rem as euclidean_div_rem;
pub use crate::math::floor_to_power_of_two;
pub use crate::math::inference::error::Error as InferenceError;
pub use crate::math::inference::literal_types as infer_literal_types;
pub use crate::math::inference::minimal_bitlength as infer_minimal_bitlength;
pub use crate::math::inference::r#type::Type as InferredType;
pub use crate::math::inference::result::Binary as BinaryInferenceResult;
pub use crate::math::log2ceil;
pub use crate::token::parse_amount as parse_token_amount;
