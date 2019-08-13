//!
//! The comment lexeme.
//!

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Comment(pub String);
