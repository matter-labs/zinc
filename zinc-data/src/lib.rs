//!
//! The Zinc source code JSON representation.
//!

pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod source;

pub use self::request::call::Body as CallRequestBody;
pub use self::request::call::Query as CallRequestQuery;
pub use self::request::publish::Body as PublishRequestBody;
pub use self::request::publish::Query as PublishRequestQuery;
pub use self::request::query::Body as QueryRequestBody;
pub use self::request::query::Query as QueryRequestQuery;
pub use self::response::publish::Body as PublishResponseBody;
pub use self::source::directory::Directory;
pub use self::source::error::Error as SourceError;
pub use self::source::file::File;
pub use self::source::Source;