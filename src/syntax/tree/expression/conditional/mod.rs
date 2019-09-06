//!
//! The block expression.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::BlockExpression;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub condition: Box<syntax::Expression>,
    pub main_block: BlockExpression,
    pub else_if: Option<Box<Self>>,
    pub else_block: Option<BlockExpression>,
}

impl Expression {
    pub fn new(
        location: Location,
        condition: syntax::Expression,
        main_block: BlockExpression,
        else_if: Option<Self>,
        else_block: Option<BlockExpression>,
    ) -> Self {
        Self {
            location,
            condition: Box::new(condition),
            main_block,
            else_if: else_if.map(Box::new),
            else_block,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if {} {}{}{}",
            self.condition,
            self.main_block,
            if let Some(ref else_if) = self.else_if {
                format!(" else {}", else_if)
            } else {
                "".to_owned()
            },
            if let Some(ref else_block) = self.else_block {
                format!(" else {}", else_block)
            } else {
                "".to_owned()
            }
        )
    }
}
