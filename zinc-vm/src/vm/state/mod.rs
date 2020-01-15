mod cell;
mod data_stack;
mod evaluation_stack;

pub use cell::*;
pub use data_stack::*;
pub use evaluation_stack::*;

use crate::gadgets::Primitive;
use pairing::Engine;

#[derive(Debug)]
pub struct Loop {
    pub first_instruction_index: usize,
    pub iterations_left: usize,
}

#[derive(Debug)]
pub struct Branch<E: Engine> {
    pub condition: Primitive<E>,
    /// False if there is only one case (If-Endif), true if two cases (If-Else-Endif).
    pub is_full: bool,
}

#[derive(Debug)]
pub enum Block<E: Engine> {
    Loop(Loop),
    Branch(Branch<E>),
}

#[derive(Debug)]
pub struct FunctionFrame<E: Engine> {
    pub blocks: Vec<Block<E>>,
    pub return_address: usize,
    pub stack_frame_begin: usize,
    pub stack_frame_end: usize,
}

#[derive(Debug)]
pub struct State<E: Engine> {
    pub instruction_counter: usize,
    pub evaluation_stack: EvaluationStack<E>,
    pub data_stack: DataStack<E>,
    pub conditions_stack: Vec<Primitive<E>>,
    pub frames_stack: Vec<FunctionFrame<E>>,
}

impl<E: Engine> FunctionFrame<E> {
    pub fn new(data_stack_address: usize, return_address: usize) -> Self {
        Self {
            blocks: vec![],
            return_address,
            stack_frame_begin: data_stack_address,
            stack_frame_end: data_stack_address,
        }
    }
}
