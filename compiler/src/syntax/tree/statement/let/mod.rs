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

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Let {
    pub location: Location,
    pub identifier: Identifier,
    pub is_mutable: bool,
    pub r#type: Option<Type>,
    pub expression: Expression,
}

impl Let {
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_mutable: bool,
        r#type: Option<Type>,
        expression: Expression,
    ) -> Self {
        Self {
            location,
            identifier,
            is_mutable,
            r#type,
            expression,
        }
    }
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let{} {}{} = {}",
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
