//!
//! The Zinc PostgreSQL library.
//!

pub(crate) mod client;
pub(crate) mod error;
pub(crate) mod model;

pub use self::client::Client;
pub use self::error::Error;
pub use self::model::entry::insert::input::Input as EntryInsertInput;
pub use self::model::entry::select::templates::Input as EntrySelectTemplatesInput;
pub use self::model::entry::select::templates::Output as EntrySelectTemplatesOutput;
pub use self::model::program::insert::input::Input as ProgramInsertInput;
pub use self::model::program::select::all::Output as ProgramSelectOutput;
pub use self::model::program::select::source::Input as ProgramSelectSourceInput;
pub use self::model::program::select::source::Output as ProgramSelectSourceOutput;
