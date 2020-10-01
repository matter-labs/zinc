//!
//! The Zandbox server daemon library.
//!

#[cfg(test)]
mod tests;

pub(crate) mod controller;
pub(crate) mod database;
pub(crate) mod response;
pub(crate) mod shared_data;

pub use self::controller::configure;
pub use self::database::client::Client as DatabaseClient;
pub use self::database::model::contract::select_all::Output as ContractSelectAllOutput;
pub use self::database::model::field::select::Input as FieldSelectInput;
pub use self::database::model::field::select::Output as FieldSelectOutput;
pub use self::shared_data::contract::Contract as SharedDataContract;
pub use self::shared_data::SharedData;

pub(crate) type Result<T, E> = ::std::result::Result<self::response::Response<T, E>, E>;
