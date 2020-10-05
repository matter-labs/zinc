//!
//! The Zinc Zandbox constants.
//!

/// The default server binding host.
pub static HOST: &str = "0.0.0.0";

/// The default server binding port.
pub const PORT: u16 = 4001;

/// The default Zargo connection url.
pub static CONNECTION_URL: &str = "https://rinkeby-zandbox.zksync.dev";

/// The default contract constructor name.
pub static CONTRACT_CONSTRUCTOR_NAME: &str = "new";

/// The contract publish URL.
pub static CONTRACT_PUBLISH_URL: &str = "/api/v1/contract";

/// The contract initialize URL.
pub static CONTRACT_INITIALIZE_URL: &str = "/api/v1/contract/initialize";

/// The contract query URL.
pub static CONTRACT_QUERY_URL: &str = "/api/v1/contract/query";

/// The contract fee URL.
pub static CONTRACT_FEE_URL: &str = "/api/v1/contract/fee";

/// The contract call URL.
pub static CONTRACT_CALL_URL: &str = "/api/v1/contract/call";

/// The ETH balance value exponent.
pub const ETH_BALANCE_EXPONENT: u32 = 18;
