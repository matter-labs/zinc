//!
//! The VM state block.
//!

pub mod branch;
pub mod r#loop;

use crate::IEngine;

use self::branch::Branch;
use self::r#loop::Loop;

#[derive(Debug)]
pub enum Block<E: IEngine> {
    Loop(Loop),
    Branch(Branch<E>),
}
