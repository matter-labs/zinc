use crate::element::{Element, ElementOperator};
use crate::vm::RuntimeError::StackUnderflow;
use num_bigint::BigInt;
use zrust_bytecode::{dispatch_instruction, Instruction, InstructionInfo};

pub trait VMInstruction<E, O>: InstructionInfo
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError>;
}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    InvalidOperation(u8),
    InvalidArguments,
    StackUnderflow,
    StackOverflow,
    UnexpectedEndOfFile,
    SynthesisError,
    InternalError,
    IntegerOverflow,
    UnexpectedLoopExit,
    UnexpectedReturn,
    UnexpectedFrameExit,
    AssertionError,
    FirstInstructionNotCall,
    WrongInputsCount,
}

#[derive(Copy, Clone)]
struct Loop {
    first_instruction_index: usize,
    iterations_left: usize,
    io_size: usize,
}

#[derive(Copy, Clone)]
struct Function {
    return_address: usize,
}

enum Scope {
    Loop(Loop),
    Function(Function),
}

#[derive(Clone)]
struct Frame {
    address: usize,
}

pub struct VirtualMachine<E: Element, O: ElementOperator<E>> {
    instruction_counter: usize,
    stack: Vec<E>,
    frames: Vec<Frame>,
    scopes: Vec<Scope>,
    operator: O,
    conditions: Vec<E>,
    outputs: Vec<E>,
}

impl<E: Element, O: ElementOperator<E>> VirtualMachine<E, O> {
    pub fn new(operator: O) -> Self {
        Self {
            instruction_counter: 0,
            stack: vec![],
            frames: vec![Frame { address: 0 }],
            scopes: vec![],
            operator,
            conditions: vec![],
            outputs: vec![],
        }
    }

    pub fn stack_push(&mut self, element: E) -> Result<(), RuntimeError> {
        self.stack.push(element);
        Ok(())
    }

    pub fn stack_pop(&mut self) -> Result<E, RuntimeError> {
        let frame = self.frames.last().ok_or(RuntimeError::StackUnderflow)?;

        if self.stack.len() > frame.address {
            self.stack.pop().ok_or(RuntimeError::InternalError)
        } else {
            Err(RuntimeError::StackUnderflow)
        }
    }

    pub fn stack_get(&self, index: usize) -> Result<E, RuntimeError> {
        let frame = self.frames.last().ok_or(RuntimeError::StackUnderflow)?;
        self.stack
            .get(frame.address + index)
            .ok_or(RuntimeError::InternalError)
            .map(|e| (*e).clone())
    }

    pub fn run(
        &mut self,
        instructions: &[Instruction],
        inputs: Option<&[BigInt]>,
    ) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let one = self.operator.constant_bigint(&1.into())?;
        self.condition_push(one)?;

        if let Some(instruction) = instructions.first() {
            self.push_inputs(instruction, inputs)?;
        }

        while self.instruction_counter < instructions.len() {
            let instruction = &instructions[self.instruction_counter];
            self.instruction_counter += 1;
            log::info!(
                "executing: {}",
                dispatch_instruction!(instruction => instruction.to_assembly())
            );
            dispatch_instruction!(instruction => instruction.execute(self))?;
            log::info!("stack: {}", self.stack_to_string());
        }

        self.get_outputs()
    }

    fn push_inputs(
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

            for value in values.iter() {
                let var = self.operator.variable_bigint(value)?;
                self.stack_push(var)?;
            }

            Ok(())
        } else {
            for _ in 0..call.inputs_count {
                let var = self.operator.variable_none()?;
                self.stack_push(var)?;
            }
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

    pub fn stack_to_string(&self) -> String {
        let mut s = String::new();
        for e in self.stack.iter().rev() {
            s += format!("{:?} ", e).as_str();
        }
        s
    }

    pub fn get_operator(&mut self) -> &mut O {
        &mut self.operator
    }

    /// Take `inputs_count` values from current frame and push them into new one.
    fn frame_push(&mut self, inputs_count: usize) -> Result<(), RuntimeError> {
        let frame = self.frames.last().ok_or(RuntimeError::StackUnderflow)?;
        let address = self
            .stack
            .len()
            .checked_sub(inputs_count)
            .ok_or(RuntimeError::StackUnderflow)?;
        if address < frame.address {
            return Err(RuntimeError::StackUnderflow);
        }
        self.frames.push(Frame { address: address });
        Ok(())
    }

    /// Drop current frame and push `outputs_count` top values into the frame below.
    fn frame_pop(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        let frame = self.frames.pop().ok_or(RuntimeError::StackUnderflow)?;
        let outputs_address = self
            .stack
            .len()
            .checked_sub(outputs_count)
            .ok_or(RuntimeError::StackUnderflow)?;

        if outputs_address < frame.address {
            return Err(RuntimeError::StackUnderflow);
        }

        let mut outputs = Vec::from(&self.stack[outputs_address..]);
        self.stack.truncate(frame.address);
        self.stack.append(&mut outputs);

        Ok(())
    }

    pub fn loop_begin(&mut self, iterations: usize, io_size: usize) -> Result<(), RuntimeError> {
        let loop_frame = Loop {
            first_instruction_index: self.instruction_counter,
            iterations_left: iterations - 1,
            io_size,
        };
        self.scopes.push(Scope::Loop(loop_frame));
        self.frame_push(io_size)?;

        Ok(())
    }

    pub fn loop_end(&mut self) -> Result<(), RuntimeError> {
        let mut frame = match self.scopes.pop() {
            Some(Scope::Loop(frame)) => Ok(frame),
            _ => Err(RuntimeError::UnexpectedLoopExit),
        }?;

        self.frame_pop(frame.io_size)?;

        if frame.iterations_left != 0 {
            frame.iterations_left -= 1;
            self.frame_push(frame.io_size)?;
            self.instruction_counter = frame.first_instruction_index;
            self.scopes.push(Scope::Loop(frame));
        }

        Ok(())
    }

    pub fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), RuntimeError> {
        let frame = Function {
            return_address: self.instruction_counter,
        };
        self.frame_push(inputs_count)?;
        self.scopes.push(Scope::Function(frame));
        self.instruction_counter = address;
        Ok(())
    }

    pub fn ret(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        let frame = match self.scopes.pop() {
            Some(Scope::Function(loop_frame)) => Ok(loop_frame),
            _ => Err(RuntimeError::UnexpectedReturn),
        }?;

        self.frame_pop(outputs_count)?;
        self.instruction_counter = frame.return_address;

        Ok(())
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

    pub fn exit(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        if self.stack.len() < outputs_count {
            return Err(StackUnderflow);
        }

        let index = self.stack.len() - outputs_count;
        self.outputs = Vec::from(&self.stack[index..]);

        self.instruction_counter = std::usize::MAX;
        Ok(())
    }
}
