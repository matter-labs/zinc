//!
//! The let statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
pub struct Let {
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: Option<Type>,
    pub expression: Expression,
    pub is_mutable: bool,
}

impl Let {
    pub fn new(
        location: Location,
        identifier: Identifier,
        r#type: Option<Type>,
        expression: Expression,
        is_mutable: bool,
    ) -> Self {
        Self {
            location,
            identifier,
            r#type,
            expression,
            is_mutable,
        }
    }
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let{} {}{} = ( {} )",
            if self.is_mutable { " mut" } else { "" },
            self.identifier,
            if let Some(ref r#type) = self.r#type {
                format!(": {}", r#type)
            } else {
                "".to_owned()
            },
            self.expression,
        )
    }
}
