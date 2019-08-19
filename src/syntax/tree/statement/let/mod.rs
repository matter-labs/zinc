//!
//! The let statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Identifier;
use crate::syntax::Expression;
use crate::syntax::Type;

#[derive(Debug, Serialize, PartialEq)]
pub struct Let {
    identifier: Identifier,
    r#type: Option<Type>,
    expression: Expression,
    is_mutable: bool,
}

impl Let {
    pub fn new(
        identifier: Identifier,
        r#type: Option<Type>,
        expression: Expression,
        is_mutable: bool,
    ) -> Self {
        Self {
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
                "".to_string()
            },
            self.expression,
        )
    }
}
