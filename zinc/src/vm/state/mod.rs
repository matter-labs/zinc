mod data_stack;
mod evaluation_stack;

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
    pub then_memory: Option<EvaluationStack<P>>,
    pub else_memory: Option<EvaluationStack<P>>,
}

#[derive(Debug)]
pub enum Block<P: Primitive> {
    Loop(Loop),
    Branch(Branch<P>),
}

#[derive(Debug)]
pub struct FunctionFrame<P: Primitive> {
    pub blocks: Vec<Block<P>>,
    pub memory_snapshots: Vec<EvaluationStack<P>>,
    pub return_address: usize,
    pub stack_frame_begin: usize,
    pub stack_frame_end: usize,
}

#[derive(Debug)]
pub struct State<P: Primitive> {
    pub instruction_counter: usize,
    pub conditions_stack: Vec<P>,
    pub data_stack: DataStack<P>,
    pub function_frames: Vec<FunctionFrame<P>>,
}


impl<P: Primitive> FunctionFrame<P> {
    pub fn new(data_stack_address: usize, return_address: usize) -> Self {
        Self {
            blocks: vec![],
            memory_snapshots: vec![EvaluationStack::new()],
            return_address,
            stack_frame_begin: data_stack_address,
            stack_frame_end: data_stack_address,
        }
    }
}
