//!
//! The semantic analyzer scope static item.
//!

use crate::semantic::element::constant::Constant;

#[derive(Debug, Clone, PartialEq)]
pub struct Static {
    pub data: Constant,
    pub address: usize,
}

impl Static {
    pub fn new(data: Constant, address: usize) -> Self {
        Self { data, address }
    }
}
