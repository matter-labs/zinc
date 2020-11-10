//!
//! The Zinc project manifest library.
//!

pub mod error;
pub mod manifest;
pub mod project_type;

pub use self::error::Error;
pub use self::manifest::Manifest;
pub use self::manifest::Project;
pub use self::project_type::ProjectType;

pub(crate) type Result<T> = std::result::Result<T, Error>;
