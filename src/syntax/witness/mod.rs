//!
//! The syntax witness.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize)]
pub struct Witness {
    identifier: Identifier,
    r#type: Type,
}
