//!
//! The Zinc source code library.
//!

pub(crate) mod error;
pub(crate) mod project;
pub(crate) mod source;

pub use self::error::Error;
pub use self::project::Project;
pub use self::source::directory::Directory;
pub use self::source::file::File;
pub use self::source::Source;
