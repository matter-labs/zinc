use crate::{Element, ElementOperator, VMInstruction};

#[derive(Debug)]
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
}

#[derive(Copy, Clone)]
struct StackFrame {
    stack_address: usize,
    io_size: usize,
}

#[derive(Copy, Clone)]
struct LoopFrame {
    first_instruction_index: usize,
    iterations_left: usize,
    io_size: usize,
}

#[derive(Copy, Clone)]
struct FunctionFrame {
    return_index: usize,
}

#[derive(Copy, Clone)]
enum Frame {
    LoopFrame(LoopFrame),
    FunctionFrame(FunctionFrame),
    StackFrame(StackFrame),
}

pub struct VirtualMachine<E, O>
where
    E: Element,
    O: ElementOperator<E>
{
    stack: Vec<E>,
    frames: Vec<Frame>,
    operator: O,
    instruction_counter: usize,
}

impl <E, O> VirtualMachine<E, O>
where
    E: Element,
    O: ElementOperator<E>
{
    pub fn new(operator: O) -> Self {
        Self {
            stack: Vec::new(),
            operator,
            instruction_counter: 0,
            frames: Vec::new(),
        }
    }

    pub fn run(&mut self, instructions: &mut [Box<dyn VMInstruction<E, O>>])
        -> Result<(), RuntimeError>
    {
        while self.instruction_counter < instructions.len() {
            let instruction = &mut instructions[self.instruction_counter];
            self.instruction_counter += 1;
            log::info!(">>> {}", instruction.to_assembly());
            instruction.execute(self)?;
            self.log_stack();
        }

        Ok(())
    }

    pub fn log_stack(&self) {
        let mut s = String::new();
        for e in self.stack.iter().rev() {
            s += format!("{} ", e).as_str();
        }
        log::info!("{}", s)
    }

    pub fn stack_push(&mut self, element: E) -> Result<(), RuntimeError> {
        self.stack.push(element);

        Ok(())
    }

    pub fn stack_pop(&mut self) -> Result<E, RuntimeError> {
        self.stack
            .pop()
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn stack_get(&self, index: usize) -> Result<E, RuntimeError> {
        self.stack
            .get(index)
            .ok_or(RuntimeError::StackUnderflow)
            .map(|e| (*e).clone())
    }

    pub fn get_operator(&mut self) -> &mut O {
        &mut self.operator
    }

    pub fn loop_begin(&mut self, iterations: usize, io_size: usize) -> Result<(), RuntimeError> {
        self.frames.push(Frame::LoopFrame(LoopFrame {
            first_instruction_index: self.instruction_counter,
            iterations_left: iterations - 1,
            io_size
        }));

        self.stack_frame_push(io_size)?;

        Ok(())
    }

    pub fn loop_end(&mut self) -> Result<(), RuntimeError> {
        self.stack_frame_pop()?;

        if let Some(Frame::LoopFrame(mut frame)) = self.frames.pop() {
            if frame.iterations_left != 0 {
                self.instruction_counter = frame.first_instruction_index;
                frame.iterations_left -= 1;
                self.frames.push(Frame::LoopFrame(frame));
                self.stack_frame_push(frame.io_size);
            }
            Ok(())
        } else {
            Err(RuntimeError::UnexpectedLoopExit)
        }
    }

    pub fn function_call(&mut self, address: usize) -> Result<(), RuntimeError> {
        let frame = FunctionFrame { return_index: self.instruction_counter };
        self.frames.push(Frame::FunctionFrame(frame));
        self.instruction_counter = address;
        Ok(())
    }

    pub fn function_return(&mut self) -> Result<(), RuntimeError> {
        if let Some(Frame::FunctionFrame(frame)) = self.frames.pop() {
            self.instruction_counter = frame.return_index;
            Ok(())
        } else {
            Err(RuntimeError::UnexpectedReturn)
        }
    }

    pub fn stack_frame_push(&mut self, io_size: usize) -> Result<(), RuntimeError> {
        if io_size > self.stack.len() {
            return Err(RuntimeError::StackUnderflow);
        }

        let address = self.stack.len() - io_size;
        self.frames.push(Frame::StackFrame(StackFrame {
            stack_address: address, io_size
        }));

        Ok(())
    }

    pub fn stack_frame_pop(&mut self) -> Result<(), RuntimeError> {
        if let Some(Frame::StackFrame(frame)) = self.frames.pop() {
            if frame.stack_address + frame.io_size > self.stack.len() {
                return Err(RuntimeError::StackUnderflow);
            }

            let output_address = self.stack.len() - frame.io_size;
            let mut output = Vec::from(&self.stack[output_address..]);
            self.stack.truncate(frame.stack_address);
            self.stack.append(&mut output);

            Ok(())
        } else {
            Err(RuntimeError::UnexpectedFrameExit)
        }
    }
}
