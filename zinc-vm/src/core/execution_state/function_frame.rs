//!
//! The VM state function frame.
//!

use crate::core::execution_state::block::Block;
use crate::IEngine;

#[derive(Debug)]
pub struct Frame<E: IEngine> {
    pub blocks: Vec<Block<E>>,
    pub return_address: usize,
    pub stack_frame_start: usize,
    pub stack_frame_end: usize,
}

impl<E: IEngine> Frame<E> {
    const BLOCKS_INITIAL_CAPACITY: usize = 16;

    pub fn new(data_stack_address: usize, return_address: usize) -> Self {
        Self {
            blocks: Vec::with_capacity(Self::BLOCKS_INITIAL_CAPACITY),
            return_address,
            stack_frame_start: data_stack_address,
            stack_frame_end: data_stack_address,
        }
    }
}
