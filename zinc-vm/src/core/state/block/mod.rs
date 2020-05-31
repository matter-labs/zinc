//!
//! The VM state block.
//!

pub mod branch;
pub mod r#loop;

use crate::Engine;

use self::branch::Branch;
use self::r#loop::Loop;

#[derive(Debug)]
pub enum Block<E: Engine> {
    Loop(Loop),
    Branch(Branch<E>),
}
