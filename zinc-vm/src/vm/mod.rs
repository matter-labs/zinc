mod internal;
mod state;

pub use internal::*;
pub use state::*;

use crate::gadgets::{Primitive, PrimitiveOperations, ScalarType};
use crate::ZincEngine;
use franklin_crypto::bellman::SynthesisError;
use num_bigint::{BigInt, ToBigInt};
use zinc_bytecode::program::Program;
use zinc_bytecode::{dispatch_instruction, Instruction, InstructionInfo};

pub trait VMInstruction<E, O>: InstructionInfo
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError>;
}

#[derive(Debug)]
pub enum RuntimeError {
    InvalidArguments,
    StackUnderflow,
    StackOverflow,
    UnexpectedEndOfFile,
    SynthesisError(SynthesisError),
    InternalError(String),
    IntegerOverflow,
    UnexpectedLoopEnd,
    UnexpectedReturn,
    UnexpectedFrameExit,
    UnexpectedElse,
    UnexpectedEndIf,
    AssertionError,
    FirstInstructionNotCall,
    WrongInputsCount,
    StackIndexOutOfRange,
    UninitializedStorageAccess,
    MissingArgument,
    BranchStacksDoNotMatch,
    IndexOutOfBounds,
    MergingNonValueTypes,
    UnexpectedNonValueType,
    OperationOnDifferentTypes,
}

impl From<SynthesisError> for RuntimeError {
    fn from(error: SynthesisError) -> Self {
        RuntimeError::SynthesisError(error)
    }
}

pub struct VirtualMachine<E: ZincEngine, O: PrimitiveOperations<E>> {
    pub(crate) debugging: bool,
    state: State<E>,
    ops: O,
    outputs: Vec<Primitive<E>>,
}

impl<E: ZincEngine, O: PrimitiveOperations<E>> VirtualMachine<E, O> {
    pub fn new(operator: O, debugging: bool) -> Self {
        Self {
            debugging,
            state: State {
                instruction_counter: 0,
                evaluation_stack: EvaluationStack::new(),
                data_stack: DataStack::new(),
                conditions_stack: vec![],
                frames_stack: vec![],
            },
            ops: operator,
            outputs: vec![],
        }
    }

    pub fn run(
        &mut self,
        program: &Program,
        inputs: Option<&[BigInt]>,
    ) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let one = self
            .ops
            .constant_bigint_typed(&1.into(), ScalarType::BOOLEAN)?;
        self.condition_push(one)?;

        match program.bytecode.first() {
            Some(Instruction::Call(call)) => {
                self.init_root_frame(call.inputs_count, inputs)?;
            }
            _ => unimplemented!("Program must start with Call instruction"),
        }

        while self.state.instruction_counter < program.bytecode.len() {
            let instruction = &program.bytecode[self.state.instruction_counter];
            self.state.instruction_counter += 1;
            log::info!(
                "> {}",
                dispatch_instruction!(instruction => instruction.to_assembly())
            );
            dispatch_instruction!(instruction => instruction.execute(self))?;
            log::info!("{}", self.state_to_string());
        }

        self.get_outputs()
    }

    fn init_root_frame(
        &mut self,
        inputs_count: usize,
        inputs: Option<&[BigInt]>,
    ) -> Result<(), RuntimeError> {
        self.state
            .frames_stack
            .push(FunctionFrame::new(0, std::usize::MAX));

        match inputs {
            None => {
                for _ in 0..inputs_count {
                    let variable = self.ops.variable_none()?;
                    self.push(Cell::Value(variable))?;
                }
            }
            Some(values) => {
                for value in values.iter() {
                    let variable = self.ops.variable_bigint(value)?;
                    self.push(Cell::Value(variable))?;
                }
            }
        }

        Ok(())
    }

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let mut outputs = Vec::new();

        for o in self.outputs.iter() {
            let e = self.ops.output(o.clone())?;
            outputs.push(e.to_bigint());
        }

        Ok(outputs)
    }

    fn state_to_string(&self) -> String {
        //        format!("{:#?}", self.state)
        "".into()
    }

    pub fn operations(&mut self) -> &mut O {
        &mut self.ops
    }

    pub fn condition_push(&mut self, element: Primitive<E>) -> Result<(), RuntimeError> {
        self.state.conditions_stack.push(element);
        Ok(())
    }

    pub fn condition_pop(&mut self) -> Result<Primitive<E>, RuntimeError> {
        self.state
            .conditions_stack
            .pop()
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn condition_top(&mut self) -> Result<Primitive<E>, RuntimeError> {
        self.state
            .conditions_stack
            .last()
            .map(|e| (*e).clone())
            .ok_or(RuntimeError::StackUnderflow)
    }

    fn top_frame(&mut self) -> Result<&mut FunctionFrame<E>, RuntimeError> {
        self.state
            .frames_stack
            .last_mut()
            .ok_or(RuntimeError::StackUnderflow)
    }
}
