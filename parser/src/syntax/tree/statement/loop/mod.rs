//!
//! The loop statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub location: Location,
    pub id: String,
    pub index_identifier: Identifier,
    pub range_start: usize,
    pub range_end: usize,
    pub is_range_inclusive: bool,
    pub while_condition: Option<Expression>,
    pub block: BlockExpression,
}

impl Loop {
    pub fn new(
        location: Location,
        index_identifier: Identifier,
        range_start: usize,
        range_end: usize,
        is_range_inclusive: bool,
        while_condition: Option<Expression>,
        block: BlockExpression,
    ) -> Self {
        let id = format!("L{}", location.line);

        Self {
            location,
            id,
            index_identifier,
            range_start,
            range_end,
            is_range_inclusive,
            while_condition,
            block,
        }
    }
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "for {} in {}{}{}{} {}",
            self.index_identifier,
            self.range_start,
            if self.is_range_inclusive { "..=" } else { ".." },
            self.range_end,
            if let Some(ref while_condition) = self.while_condition {
                format!(" while {}", while_condition)
            } else {
                "".to_owned()
            },
            self.block,
        )
    }
}
