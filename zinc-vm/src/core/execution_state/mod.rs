//!
//! The VM execution state.
//!

pub mod block;
pub mod cell;
pub mod data_stack;
pub mod evaluation_stack;
pub mod function_frame;

use std::fmt;

use crate::core::contract::output::initializer::Initializer;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

use self::data_stack::DataStack;
use self::evaluation_stack::EvaluationStack;
use self::function_frame::Frame;

#[derive(Debug)]
pub struct ExecutionState<E: IEngine> {
    pub instruction_counter: usize,
    pub evaluation_stack: EvaluationStack<E>,
    pub data_stack: DataStack<E>,
    pub conditions_stack: Vec<Scalar<E>>,
    pub frames_stack: Vec<Frame<E>>,
    pub transfers: Vec<zinc_types::TransactionMsg>,
    pub initializers: Vec<Initializer>,
}

impl<E: IEngine> ExecutionState<E> {
    const CONDITIONS_INITIAL_CAPACITY: usize = 16;
    const FRAMES_INITIAL_CAPACITY: usize = 16;
    const TRANSFERS_INITIAL_CAPACITY: usize = 4;
    const INITIALIZERS_INITIAL_CAPACITY: usize = 4;

    pub fn new() -> Self {
        Self {
            instruction_counter: 0,
            evaluation_stack: EvaluationStack::new(),
            data_stack: DataStack::new(),
            conditions_stack: Vec::with_capacity(Self::CONDITIONS_INITIAL_CAPACITY),
            frames_stack: Vec::with_capacity(Self::FRAMES_INITIAL_CAPACITY),
            transfers: Vec::with_capacity(Self::TRANSFERS_INITIAL_CAPACITY),
            initializers: Vec::with_capacity(Self::INITIALIZERS_INITIAL_CAPACITY),
        }
    }
}

impl<E: IEngine> fmt::Display for ExecutionState<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.evaluation_stack)?;
        writeln!(
            f,
            "Data stack offset: {}\n",
            self.frames_stack
                .last()
                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                .stack_frame_start
        )?;
        writeln!(f, "{}", self.data_stack)?;

        Ok(())
    }
}
