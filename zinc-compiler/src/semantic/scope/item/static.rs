//!
//! The semantic analyzer scope static item.
//!

use crate::semantic;

#[derive(Debug, Clone, PartialEq)]
pub struct Static {
    pub data: semantic::Constant,
    pub address: usize,
}

impl Static {
    pub fn new(data: semantic::Constant, address: usize) -> Self {
        Self { data, address }
    }
}
