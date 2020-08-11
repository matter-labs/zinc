//!
//! The Zinc PostgreSQL library.
//!

pub(crate) mod client;
pub(crate) mod error;
pub(crate) mod model;

pub use self::client::Client;
pub use self::error::Error;
pub use self::model::method::insert::input::Input as MethodInsertInput;
pub use self::model::method::select::types::Input as MethodSelectTypesInput;
pub use self::model::method::select::types::Output as MethodSelectTypesOutput;
pub use self::model::template::insert::input::Input as TemplateInsertInput;
pub use self::model::template::select::single::Input as TemplateSelectInput;
pub use self::model::template::select::single::Output as TemplateSelectOutput;
