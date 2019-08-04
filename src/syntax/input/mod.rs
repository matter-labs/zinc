//!
//! The syntax input.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize)]
pub struct Input {
    identifier: Identifier,
    r#type: Type,
}
