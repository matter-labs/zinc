mod cell;
mod data_stack;
mod evaluation_stack;

pub use cell::*;
pub use data_stack::*;
pub use evaluation_stack::*;

use crate::primitive::Primitive;

#[derive(Debug)]
pub struct Loop {
    pub first_instruction_index: usize,
    pub iterations_left: usize,
}

#[derive(Debug)]
pub struct Branch<P: Primitive> {
    pub condition: P,
    /// False if there is only one case (If-Endif), true if two cases (If-Else-Endif).
    pub is_full: bool,
}

#[derive(Debug)]
pub enum Block<P: Primitive> {
    Loop(Loop),
    Branch(Branch<P>),
}

#[derive(Debug)]
pub struct FunctionFrame<P: Primitive> {
    pub blocks: Vec<Block<P>>,
    pub return_address: usize,
    pub stack_frame_begin: usize,
    pub stack_frame_end: usize,
}

#[derive(Debug)]
pub struct State<P: Primitive> {
    pub instruction_counter: usize,
    pub evaluation_stack: EvaluationStack<P>,
    pub data_stack: DataStack<P>,
    pub conditions_stack: Vec<P>,
    pub frames_stack: Vec<FunctionFrame<P>>,
}

impl<P: Primitive> FunctionFrame<P> {
    pub fn new(data_stack_address: usize, return_address: usize) -> Self {
        Self {
            blocks: vec![],
            return_address,
            stack_frame_begin: data_stack_address,
            stack_frame_end: data_stack_address,
        }
    }
}
