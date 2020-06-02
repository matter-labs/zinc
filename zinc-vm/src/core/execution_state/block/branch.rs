//!
//! The VM state conditional branch block.
//!

use crate::gadgets::scalar::Scalar;
use crate::IEngine;

#[derive(Debug)]
pub struct Branch<E: IEngine> {
    pub condition: Scalar<E>,
    pub is_else: bool,
}
