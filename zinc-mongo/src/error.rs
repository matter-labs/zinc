//!
//! The Zinc Mongo error.
//!

use failure::Fail;

///
/// The Zinc Mongo wrapper error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The MongoDB internal error.
    #[fail(display = "MongoDb: {}", _0)]
    MongoDb(mongodb::error::Error),
    /// The record is not present in the database.
    #[fail(display = "record not found")]
    RecordNotFound,
}

impl From<mongodb::error::Error> for Error {
    fn from(inner: mongodb::error::Error) -> Self {
        Self::MongoDb(inner)
    }
}
