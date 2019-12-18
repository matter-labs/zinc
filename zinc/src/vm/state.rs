use crate::primitive::Primitive;
use crate::vm::memory::Memory;
use crate::vm::data_stack::DataStack;

#[derive(Debug)]
pub struct Loop {
    pub first_instruction_index: usize,
    pub iterations_left: usize,
}

#[derive(Debug)]
pub struct Branch<P: Primitive> {
    pub condition: P,
    pub then_memory: Option<Memory<P>>,
    pub else_memory: Option<Memory<P>>,
}

#[derive(Debug)]
pub enum Block<P: Primitive> {
    Loop(Loop),
    Branch(Branch<P>),
}

#[derive(Debug)]
pub struct FunctionFrame<P: Primitive> {
    pub blocks: Vec<Block<P>>,
    pub memory_snapshots: Vec<Memory<P>>,
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
    pub fn new(data_stack_address: usize, return_address: usize, arguments: &[P]) -> Self {
        Self {
            blocks: vec![],
            memory_snapshots: vec![Memory::new(arguments)],
            return_address,
            stack_frame_begin: data_stack_address,
            stack_frame_end: data_stack_address,
        }
    }
}
