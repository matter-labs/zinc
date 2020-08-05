//!
//! The Zinc server library.
//!

pub(crate) mod controller;
pub(crate) mod response;
pub(crate) mod shared_data;

pub use self::controller::configure;
pub use self::shared_data::SharedData;
