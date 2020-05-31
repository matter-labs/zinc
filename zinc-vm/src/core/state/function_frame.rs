//!
//! The VM state function frame.
//!

use crate::core::state::block::Block;
use crate::Engine;

#[derive(Debug)]
pub struct Frame<E: Engine> {
    pub blocks: Vec<Block<E>>,
    pub return_address: usize,
    pub stack_frame_start: usize,
    pub stack_frame_end: usize,
}

impl<E: Engine> Frame<E> {
    pub fn new(data_stack_address: usize, return_address: usize) -> Self {
        Self {
            blocks: vec![],
            return_address,
            stack_frame_start: data_stack_address,
            stack_frame_end: data_stack_address,
        }
    }
}
