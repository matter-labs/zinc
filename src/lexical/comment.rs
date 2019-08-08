//!
//! The comment lexeme.
//!

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct Comment(pub String);
