//!
//! The VM state conditional branch block.
//!

use crate::gadgets::scalar::Scalar;
use crate::Engine;

#[derive(Debug)]
pub struct Branch<E: Engine> {
    pub condition: Scalar<E>,
    pub is_else: bool,
}
