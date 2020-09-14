//!
//! The Zinc source code JSON representation.
//!

pub(crate) mod source;

pub use self::source::directory::Directory;
pub use self::source::error::Error as SourceError;
pub use self::source::file::File;
pub use self::source::requests::call::Body as CallRequestBody;
pub use self::source::requests::call::Query as CallRequestQuery;
pub use self::source::requests::publish::Body as PublishRequestBody;
pub use self::source::requests::publish::Query as PublishRequestQuery;
pub use self::source::requests::query::Body as QueryRequestBody;
pub use self::source::requests::query::Query as QueryRequestQuery;
pub use self::source::Source;
