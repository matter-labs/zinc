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
}

pub struct VirtualMachine<E, O>
where
    E: Element,
    O: ElementOperator<E>
{
    stack: Vec<E>,
    operator: O,
    instruction_counter: usize,
    loop_stack: Vec<LoopFrame>,
}

struct LoopFrame {
    first_instruction_index: usize,
    iterations_left: usize,
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
            loop_stack: Vec::new(),
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
        let last = self.stack.len();
        self.stack
            .get(last - index - 1)
            .ok_or(RuntimeError::StackUnderflow)
            .map(|e| (*e).clone())
    }

    pub fn get_operator(&mut self) -> &mut O {
        &mut self.operator
    }

    pub fn loop_begin(&mut self, iterations: usize) -> Result<(), RuntimeError> {
        self.loop_stack.push(LoopFrame {
            first_instruction_index: self.instruction_counter,
            iterations_left: iterations - 1,
        });

        Ok(())
    }

    pub fn loop_end(&mut self) -> Result<(), RuntimeError> {
        let mut frame = self.loop_stack.pop().ok_or(RuntimeError::UnexpectedLoopExit)?;

        if frame.iterations_left != 0 {
            self.instruction_counter = frame.first_instruction_index;
            frame.iterations_left -= 1;
            self.loop_stack.push(frame);
        }

        Ok(())
    }
}
