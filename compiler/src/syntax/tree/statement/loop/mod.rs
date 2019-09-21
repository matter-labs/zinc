//!
//! The loop statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Identifier;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Loop {
    #[serde(skip_serializing)]
    pub location: Location,
    pub index_identifier: Identifier,
    pub range_start: IntegerLiteral,
    pub range_end: IntegerLiteral,
    pub block: BlockExpression,
}

impl Loop {
    pub fn new(
        location: Location,
        index_identifier: Identifier,
        range_start: IntegerLiteral,
        range_end: IntegerLiteral,
        block: BlockExpression,
    ) -> Self {
        Self {
            location,
            index_identifier,
            range_start,
            range_end,
            block,
        }
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "for {} in {}..{} {{ {} }}",
            self.index_identifier, self.range_start, self.range_end, self.block,
        )
    }
}
