//!
//! The Zandbox server daemon library.
//!

pub(crate) mod contract;
pub(crate) mod controller;
pub(crate) mod database;
pub(crate) mod error;
pub(crate) mod response;
pub(crate) mod shared_data;
pub(crate) mod storage;

pub use self::controller::configure;
pub use self::database::client::Client as DatabaseClient;
pub use self::error::Error;
pub use self::shared_data::SharedData;

///
/// The Actix response type anti-boilerplate wrapper.
///
pub(crate) type Result<T, E> = ::std::result::Result<self::response::Response<T, E>, E>;

///
/// The Actix shared data anti-boilerplate wrapper.
///
pub(crate) type WebData = actix_web::web::Data<std::sync::RwLock<SharedData>>;
