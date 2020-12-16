//!
//! The Zinc project representation library.
//!

pub(crate) mod error;
pub(crate) mod manifest;
pub(crate) mod project;
pub(crate) mod source;

pub use self::error::Error;
pub use self::manifest::Manifest;
pub use self::manifest::Project as ManifestProject;
pub use self::project::r#type::Type as ProjectType;
pub use self::project::Project;
pub use self::source::directory::Directory;
pub use self::source::file::File;
pub use self::source::Source;
