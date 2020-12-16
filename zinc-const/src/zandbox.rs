//!
//! The Zinc Zandbox constants.
//!

/// The default server binding host.
pub static HOST: &str = "0.0.0.0";

/// The default server binding port.
pub const PORT: u16 = 4001;

/// The project default URL.
pub static PROJECT_URL: &str = "/api/v1/project";

/// The project source URL.
pub static PROJECT_SOURCE_URL: &str = "/api/v1/project/source";

/// The contract default URL.
pub static CONTRACT_URL: &str = "/api/v1/contract";

/// The contract initialize URL.
pub static CONTRACT_INITIALIZE_URL: &str = "/api/v1/contract/initialize";

/// The contract query URL.
pub static CONTRACT_QUERY_URL: &str = "/api/v1/contract/query";

/// The contract fee URL.
pub static CONTRACT_FEE_URL: &str = "/api/v1/contract/fee";

/// The contract call URL.
pub static CONTRACT_CALL_URL: &str = "/api/v1/contract/call";
