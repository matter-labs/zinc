mod memory;

use crate::element::{Element, ElementOperator};
use num_bigint::BigInt;
use zrust_bytecode::{dispatch_instruction, Instruction, InstructionInfo};
use franklin_crypto::bellman::SynthesisError;
use crate::vm::memory::Memory;

pub trait VMInstruction<E, O>: InstructionInfo
where
    E: Element,
    O: ElementOperator<E>,
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
    UnexpectedLoopExit,
    UnexpectedReturn,
    UnexpectedFrameExit,
    AssertionError,
    FirstInstructionNotCall,
    WrongInputsCount,
    StackIndexOutOfRange,
    UninitializedStorageAccess,
    MissingArgument,
}

#[derive(Debug, Copy, Clone)]
struct Loop {
    first_instruction_index: usize,
    iterations_left: usize,
    io_size: usize,
}

#[derive(Debug)]
enum Block {
    Loop(Loop),
}

#[derive(Debug)]
struct Frame<E: Element> {
    memory: Memory<E>,
    return_address: usize,
    blocks: Vec<Block>
}

pub struct VirtualMachine<E: Element, O: ElementOperator<E>> {
    instruction_counter: usize,
    frames: Vec<Frame<E>>,
    operator: O,
    conditions: Vec<E>,
    outputs: Vec<E>,
}

impl<E: Element, O: ElementOperator<E>> VirtualMachine<E, O> {
    pub fn new(operator: O) -> Self {
        Self {
            instruction_counter: 0,
            frames: vec![],
            operator,
            conditions: vec![],
            outputs: vec![],
        }
    }

    pub fn run(
        &mut self,
        instructions: &[Instruction],
        inputs: Option<&[BigInt]>,
    ) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let one = self.operator.constant_bigint(&1.into())?;
        self.condition_push(one)?;

        if let Some(instruction) = instructions.first() {
            self.push_root_frame(instruction, inputs)?;
        }

        while self.instruction_counter < instructions.len() {
            let instruction = &instructions[self.instruction_counter];
            self.instruction_counter += 1;
            log::info!(
                "> {}",
                dispatch_instruction!(instruction => instruction.to_assembly())
            );
            dispatch_instruction!(instruction => instruction.execute(self))?;
            log::info!("{}", self.stack_to_string());
        }

        self.get_outputs()
    }

    fn push_root_frame(
        &mut self,
        instruction: &Instruction,
        inputs: Option<&[BigInt]>,
    ) -> Result<(), RuntimeError> {
        let call = match instruction {
            Instruction::Call(call) => Ok(call),
            _ => Err(RuntimeError::FirstInstructionNotCall),
        }?;

        if let Some(values) = inputs {
            if values.len() != call.inputs_count {
                return Err(RuntimeError::WrongInputsCount);
            }

            let mut arguments = Vec::new();
            for value in values.iter() {
                let var = self.operator.variable_bigint(value)?;
                arguments.push(var);
            }
            self.frames.push(Frame {
                memory: Memory::new(arguments.as_slice()),
                return_address: 0,
                blocks: vec![]
            });

            Ok(())
        } else {
            let mut arguments = Vec::new();
            for _ in 0..call.inputs_count {
                let var = self.operator.variable_none()?;
                arguments.push(var);
            }
            self.frames.push(Frame {
                memory: Memory::new(arguments.as_slice()),
                return_address: 0,
                blocks: vec![]
            });

            Ok(())
        }
    }

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let mut outputs = Vec::new();

        for o in self.outputs.iter() {
            let e = self.operator.output(o.clone())?;
            outputs.push(e.to_bigint());
        }

        Ok(outputs)
    }

    fn stack_to_string(&self) -> String {
        format!("{:?}", self.frames.last())
    }

    pub fn get_operator(&mut self) -> &mut O {
        &mut self.operator
    }


    pub fn loop_begin(&mut self, _iterations: usize, _io_size: usize) -> Result<(), RuntimeError> {
       unimplemented!("loop_begin")
    }

    pub fn loop_end(&mut self) -> Result<(), RuntimeError> {
        unimplemented!("loop_end")
    }

    pub fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), RuntimeError> {
        let frame = self.frames.last_mut().unwrap();

        let mut arguments = Vec::new();
        for _ in 0..inputs_count {
            let arg = frame.memory.pop()?;
            arguments.push(arg);
        }

        self.frames.push(Frame {
            memory: Memory::new(arguments.as_slice()),
            return_address: self.instruction_counter,
            blocks: vec![]
        });
        self.instruction_counter = address;
        Ok(())
    }

    pub fn ret(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        let mut old_frame = match self.frames.pop() {
            Some(frame) => Ok(frame),
            None => Err(RuntimeError::InternalError("Root frame is missing".into())),
        }?;

        let mut outputs = Vec::new();
        for _ in 0..outputs_count {
            let output = old_frame.memory.pop()?;
            outputs.push(output);
        }

        match self.frames.last_mut() {
            Some(frame) => {
                for v in outputs.iter() {
                    frame.memory.push((*v).clone())?;
                }
                self.instruction_counter = old_frame.return_address;
                Ok(())
            },
            None => Err(RuntimeError::UnexpectedReturn),
        }
    }

    pub fn condition_push(&mut self, element: E) -> Result<(), RuntimeError> {
        self.conditions.push(element);
        Ok(())
    }

    pub fn condition_pop(&mut self) -> Result<E, RuntimeError> {
        self.conditions.pop().ok_or(RuntimeError::StackUnderflow)
    }

    pub fn condition_top(&mut self) -> Result<E, RuntimeError> {
        self.conditions
            .last()
            .map(|e| (*e).clone())
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn frame(&mut self) -> Result<&mut Memory<E>, RuntimeError> {
        let frame = self.frames.last_mut().ok_or(RuntimeError::StackUnderflow)?;
        Ok(&mut frame.memory)
    }

    pub fn exit(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        let mut old_frame = match self.frames.pop() {
            Some(frame) => Ok(frame),
            None => Err(RuntimeError::InternalError("Root frame is missing".into())),
        }?;

        let mut outputs = Vec::new();
        for _ in 0..outputs_count {
            let output = old_frame.memory.pop()?;
            outputs.push(output);
        }

        self.outputs = outputs;
        self.instruction_counter = std::usize::MAX;
        Ok(())
    }
}
