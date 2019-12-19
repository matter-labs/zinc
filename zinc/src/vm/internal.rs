use crate::primitive::{Primitive, PrimitiveOperations};
use crate::RuntimeError;
use crate::vm::{VirtualMachine, Branch, Block, Loop, FunctionFrame, Cell};

/// This is an internal interface to virtual machine used by instructions.
pub trait InternalVM<E: Primitive> {
    fn push(&mut self, element: E) -> Result<(), RuntimeError>;
    fn pop(&mut self) -> Result<E, RuntimeError>;

    fn load(&mut self, address: usize) -> Result<E, RuntimeError>;
    fn store(&mut self, address: usize, element: E) -> Result<(), RuntimeError>;

    fn loop_begin(&mut self, iter_count: usize) -> Result<(), RuntimeError>;
    fn loop_end(&mut self) -> Result<(), RuntimeError>;

    fn call(&mut self, address: usize, inputs_count: usize) -> Result<(), RuntimeError>;
    fn ret(&mut self, outputs_count: usize) -> Result<(), RuntimeError>;

    fn branch_then(&mut self) -> Result<(), RuntimeError>;
    fn branch_else(&mut self) -> Result<(), RuntimeError>;
    fn branch_end(&mut self) -> Result<(), RuntimeError>;

    fn exit(&mut self, values_count: usize) -> Result<(), RuntimeError>;
}

impl<P, O> InternalVM<P> for VirtualMachine<P, O>
    where
        P: Primitive,
        O: PrimitiveOperations<P>,
{
    fn push(&mut self, element: P) -> Result<(), RuntimeError> {
        self.state.evaluation_stack.push(Cell::Value(element))
    }

    fn pop(&mut self) -> Result<P, RuntimeError> {
        let cell = self.state.evaluation_stack.pop()?;
        cell.value()
    }

    fn load(&mut self, address: usize) -> Result<P, RuntimeError> {
        let offset = self.top_frame()?.stack_frame_begin;
        self.state.data_stack.get(offset + address)?.value()
    }

    fn store(&mut self, address: usize, element: P) -> Result<(), RuntimeError> {
        {
            let frame = self.top_frame()?;
            frame.stack_frame_end = std::cmp::max(frame.stack_frame_end, address + 1);
        }
        let offset = self.top_frame()?.stack_frame_begin;
        self.state.data_stack.set(offset + address, Cell::Value(element))
    }

    fn loop_begin(&mut self, iterations: usize) -> Result<(), RuntimeError> {
        let frame = self.state.frames_stack
            .last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        frame.blocks.push(Block::Loop(Loop {
            first_instruction_index: self.state.instruction_counter,
            iterations_left: iterations - 1,
        }));

        Ok(())
    }

    fn loop_end(&mut self) -> Result<(), RuntimeError> {
        let frame = self.state.frames_stack.last_mut().unwrap();

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

        let offset = self.top_frame()?.stack_frame_end;
        self.state.frames_stack.push(FunctionFrame::new(
            offset,
            self.state.instruction_counter,
        ));

        for (i, p) in arguments.into_iter().enumerate() {
            self.store(i, p)?;
        }
        self.state.instruction_counter = address;
        Ok(())
    }

    fn ret(&mut self, outputs_count: usize) -> Result<(), RuntimeError> {
        let mut outputs = Vec::new();
        for _ in 0..outputs_count {
            let output = self.pop()?;
            outputs.push(output);
        }

        let frame = self.state.frames_stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        self.state.instruction_counter = frame.return_address;

        for p in outputs.into_iter() {
            self.push(p)?;
        }

        Ok(())
    }

    fn branch_then(&mut self) -> Result<(), RuntimeError> {
        let condition = self.pop()?;

        let prev = self.condition_top()?;

        let next = self.ops.and(condition.clone(), prev)?;
        self.state.conditions_stack.push(next);

        let branch = Branch { condition, is_full: false };

        self.top_frame()?.blocks.push(Block::Branch(branch));

        self.state.evaluation_stack.fork();
        self.state.data_stack.fork();

        Ok(())
    }

    fn branch_else(&mut self) -> Result<(), RuntimeError> {
        let frame = self.state.frames_stack.last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        let mut branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(RuntimeError::UnexpectedElse),
        }?;

        if branch.is_full {
            return Err(RuntimeError::UnexpectedElse);
        } else {
            branch.is_full = true;
        }

        let condition = branch.condition.clone();

        frame.blocks.push(Block::Branch(branch));

        self.condition_pop()?;
        let prev = self.condition_top()?;
        let not_cond = self.ops.not(condition)?;
        let next = self.ops.and(prev, not_cond)?;
        self.condition_push(next)?;

        self.state.data_stack.switch_branch()?;
        self.state.evaluation_stack.fork();

        Ok(())
    }

    fn branch_end(&mut self) -> Result<(), RuntimeError> {
        self.condition_pop()?;

        let frame = self.state.frames_stack.last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        let branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(RuntimeError::UnexpectedEndIf),
        }?;

        if branch.is_full {
            self.state.evaluation_stack.merge(branch.condition.clone(), &mut self.ops)?;
        } else {
            self.state.evaluation_stack.revert()?;
        }

        self.state.data_stack.merge(branch.condition, &mut self.ops)?;

        Ok(())
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
