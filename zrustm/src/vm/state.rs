use crate::element::Element;
use crate::vm::memory::Memory;

#[derive(Debug)]
pub struct Loop {
    pub first_instruction_index: usize,
    pub iterations_left: usize,
}

#[derive(Debug)]
pub struct Branch<E: Element> {
    pub condition: E,
    pub then_memory: Option<Memory<E>>,
    pub else_memory: Option<Memory<E>>,
}

#[derive(Debug)]
pub enum Block<E: Element> {
    Loop(Loop),
    Branch(Branch<E>),
}

#[derive(Debug)]
pub struct FunctionFrame<E: Element> {
    pub blocks: Vec<Block<E>>,
    pub memory_snapshots: Vec<Memory<E>>,
    pub return_address: usize,
}

#[derive(Debug)]
pub struct State<E: Element> {
    pub instruction_counter: usize,
    pub conditions_stack: Vec<E>,
    pub function_frames: Vec<FunctionFrame<E>>,
}


impl<E: Element> FunctionFrame<E> {
    pub fn new(return_address: usize, arguments: &[E]) -> Self {
        Self {
            blocks: vec![],
            memory_snapshots: vec![Memory::new(arguments)],
            return_address,
        }
    }
}
