//!
//! The Zinc Zandbox constants.
//!

/// The JSON payload limit to fit large contract source code.
pub static JSON_PAYLOAD_LIMIT: usize = 16 * 1024 * 1024;

/// The default contract constructor name.
pub static CONTRACT_CONSTRUCTOR_NAME: &str = "new";

/// The contract publish URL.
pub static CONTRACT_PUBLISH_URL: &str = "http://localhost/api/v1/contract";

/// The contract query URL.
pub static CONTRACT_QUERY_URL: &str = "http://localhost/api/v1/contract/query";

/// The contract call URL.
pub static CONTRACT_CALL_URL: &str = "http://localhost/api/v1/contract/call";
