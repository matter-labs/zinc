//!
//! The Zinc Mongo library.
//!

use futures::Future;

pub(crate) mod client;
pub(crate) mod error;
pub(crate) mod storage;

pub use self::client::Client;
pub use self::error::Error;
pub use self::storage::Storage;

///
/// Helps to turn an `async` function into a sync one.
///
pub fn wait<F: Future>(future: F) -> F::Output {
    futures::executor::block_on(future)
}
