use crate::element::{Element, ElementOperator};
use crate::RuntimeError;
use crate::vm::{VirtualMachine, Branch, Block, Loop, FunctionFrame};

/// This is an internal interface to virtual machine used by instructions.
pub trait InternalVM<E: Element> {
    fn push(&mut self, element: E) -> Result<(), RuntimeError>;
    fn pop(&mut self) -> Result<E, RuntimeError>;

    fn store(&mut self, address: usize, element: E) -> Result<(), RuntimeError>;
    fn load(&mut self, address: usize) -> Result<E, RuntimeError>;

    fn loop_begin(&mut self, iter_count: usize) -> Result<(), RuntimeError>;
    fn loop_end(&mut self) -> Result<(), RuntimeError>;

    fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), RuntimeError>;
    fn ret(&mut self, outputs_count: usize) -> Result<(), RuntimeError>;

    fn branch_then(&mut self) -> Result<(), RuntimeError>;
    fn branch_else(&mut self) -> Result<(), RuntimeError>;
    fn branch_end(&mut self) -> Result<(), RuntimeError>;

    fn exit(&mut self, values_count: usize) -> Result<(), RuntimeError>;
}

impl<E, O> InternalVM<E> for VirtualMachine<E, O>
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn push(&mut self, element: E) -> Result<(), RuntimeError> {
        self.memory()?.push(element)
    }

    fn pop(&mut self) -> Result<E, RuntimeError> {
        self.memory()?.pop()
    }

    fn store(&mut self, address: usize, element: E) -> Result<(), RuntimeError> {
        unimplemented!()
    }

    fn load(&mut self, address: usize) -> Result<E, RuntimeError> {
        unimplemented!()
    }

    fn loop_begin(&mut self, iterations: usize) -> Result<(), RuntimeError> {
        let frame = self.state.function_frames
            .last_mut()
            .ok_or(RuntimeError::InternalError("Root frame is missing".into()))?;

        frame.blocks.push(Block::Loop(Loop {
            first_instruction_index: self.state.instruction_counter,
            iterations_left: iterations - 1,
        }));

        Ok(())
    }

    fn loop_end(&mut self) -> Result<(), RuntimeError> {
        let frame = self.state.function_frames.last_mut().unwrap();

        match frame.blocks.pop() {
            Some(Block::Loop(mut loop_block)) => {
                if loop_block.iterations_left != 0 {
                    loop_block.iterations_left -= 1;
                    self.state.instruction_counter = loop_block.first_instruction_index;
                    frame.blocks.push(Block::Loop(loop_block));
                }
                Ok(())
            },
            _ => {
                Err(RuntimeError::UnexpectedLoopEnd)
            }
        }
    }

    fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), RuntimeError> {
        let mut arguments = Vec::new();
        for _ in 0..inputs_count {
            let arg = self.pop()?;
            arguments.push(arg);
        }

        self.state.function_frames.push(FunctionFrame::new(
            self.state.instruction_counter,
            arguments.as_slice(),
        ));
        self.state.instruction_counter = address;
        Ok(())
    }

    fn ret(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        let mut outputs = Vec::new();
        for _ in 0..outputs_count {
            let output = self.pop()?;
            outputs.push(output);
        }

        let frame = self.state.function_frames
            .pop()
            .ok_or(RuntimeError::UnexpectedReturn)?;

        self.state.instruction_counter = frame.return_address;

        for v in outputs.into_iter() {
            self.push(v)?;
        }

        Ok(())
    }

    fn branch_then(&mut self) -> Result<(), RuntimeError> {
        let condition = self.pop()?;
        let branch = Branch {
            condition,
            then_memory: Some(self.memory()?.fork()),
            else_memory: None
        };

        Ok(())
    }

    fn branch_else(&mut self) -> Result<(), RuntimeError> {
        unimplemented!()
    }

    fn branch_end(&mut self) -> Result<(), RuntimeError> {
        unimplemented!()
    }

    fn exit(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        for _ in 0..outputs_count {
            let value = self.pop()?;
            self.outputs.push(value);
        }

        self.state.instruction_counter = std::usize::MAX;
        Ok(())
    }
}
