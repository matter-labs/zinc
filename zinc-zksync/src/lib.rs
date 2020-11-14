//!
//! The Zinc source code JSON representation.
//!

pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod transaction;
pub(crate) mod utils;

pub use self::request::call::Body as CallRequestBody;
pub use self::request::call::Query as CallRequestQuery;
pub use self::request::fee::Body as FeeRequestBody;
pub use self::request::fee::Query as FeeRequestQuery;
pub use self::request::initialize::Body as InitializeRequestBody;
pub use self::request::initialize::Query as InitializeRequestQuery;
pub use self::request::publish::Body as PublishRequestBody;
pub use self::request::publish::Query as PublishRequestQuery;
pub use self::request::query::Body as QueryRequestBody;
pub use self::request::query::Query as QueryRequestQuery;
pub use self::request::source::Query as SourceRequestQuery;
pub use self::response::fee::Body as FeeResponseBody;
pub use self::response::initialize::Body as InitializeResponseBody;
pub use self::response::publish::Body as PublishResponseBody;
pub use self::response::source::Body as SourceResponseBody;
pub use self::transaction::error::Error as TransactionError;
pub use self::transaction::msg::Msg as TransactionMsg;
pub use self::transaction::Transaction;
pub use self::utils::eth_address_from_vec;
pub use self::utils::eth_private_key_from_vec;
pub use self::utils::num_compat_backward;
pub use self::utils::num_compat_forward;
