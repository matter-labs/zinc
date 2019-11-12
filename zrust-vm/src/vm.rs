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
}

pub struct VirtualMachine<E, O>
where
    E: Element,
    O: ElementOperator<E>
{
    stack: Vec<E>,
    operator: O,
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
        }
    }

    pub fn run(&mut self, instructions: &mut [Box<dyn VMInstruction<E, O>>])
        -> Result<(), RuntimeError>
    {
        for instr in instructions.iter_mut() {
            log::info!(">>> {}", instr.to_assembly());
            instr.execute(self)?;
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

    pub fn stack_get(&mut self, index: usize) -> Result<E, RuntimeError> {
        let last = self.stack.len();
        self.stack
            .get(last - index - 1)
            .ok_or(RuntimeError::StackUnderflow)
            .map(|e| (*e).clone())
    }

    pub fn get_operator(&mut self) -> &mut O {
        &mut self.operator
    }
}
