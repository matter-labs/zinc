use crate::primitive::{Primitive, PrimitiveOperations};
use crate::RuntimeError;
use crate::vm::{VirtualMachine, Branch, Block, Loop, FunctionFrame};
use crate::vm::data_stack::Cell;

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
        self.memory()?.push(element)
    }

    fn pop(&mut self) -> Result<P, RuntimeError> {
        self.memory()?.pop()
    }

    fn load(&mut self, address: usize) -> Result<P, RuntimeError> {
        let offset = self.frame()?.stack_frame_begin;
        match self.state.data_stack.get(offset + address) {
            Ok(cell) => {
                match cell {
                    Cell::Value(v) => Ok(v),
                    Cell::Address(_) => {
                        unimplemented!()
                    },
                }
            },
            Err(err) => Err(err),
        }
    }

    fn store(&mut self, address: usize, element: P) -> Result<(), RuntimeError> {
        {
            let frame = self.frame()?;
            frame.stack_frame_end = std::cmp::max(frame.stack_frame_end, address + 1);
        }
        let offset = self.frame()?.stack_frame_begin;
        self.state.data_stack.set(offset + address, Cell::Value(element))
    }

    fn loop_begin(&mut self, iterations: usize) -> Result<(), RuntimeError> {
        let frame = self.state.function_frames
            .last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

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

        let offset = self.frame()?.stack_frame_end;
        self.state.function_frames.push(FunctionFrame::new(
            offset,
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

        let frame = self.frame()?;

        self.state.instruction_counter = frame.return_address;

        for v in outputs.into_iter() {
            self.push(v)?;
        }

        Ok(())
    }

    fn branch_then(&mut self) -> Result<(), RuntimeError> {
        let condition = self.pop()?;

        let prev = self.condition_top()?;

        let next = self.operator.and(condition.clone(), prev)?;
        self.state.conditions_stack.push(next);

        let frame = self.state.function_frames.last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        let fork = frame.memory_snapshots.last()
            .ok_or_else(|| RuntimeError::InternalError("Root block is missing".into()))?
            .fork();

        frame.memory_snapshots.push(fork);

        let branch = Branch {
            condition,
            then_memory: None,
            else_memory: None
        };

        frame.blocks.push(Block::Branch(branch));

        Ok(())
    }

    fn branch_else(&mut self) -> Result<(), RuntimeError> {
        let frame = self.state.function_frames.last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        let mut branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(RuntimeError::UnexpectedElse),
        }?;

        if branch.then_memory.is_some() {
            return Err(RuntimeError::UnexpectedElse)
        }

        branch.then_memory = frame.memory_snapshots.pop();

        let fork = frame.memory_snapshots.last()
            .ok_or_else(|| RuntimeError::InternalError("Root block is missing".into()))?
            .fork();

        let condition = branch.condition.clone();

        frame.memory_snapshots.push(fork);
        frame.blocks.push(Block::Branch(branch));

        self.condition_pop()?;
        let prev = self.condition_top()?;
        let not_cond = self.operator.not(condition)?;
        let next = self.operator.and(prev, not_cond)?;
        self.condition_push(next)?;

        Ok(())
    }

    fn branch_end(&mut self) -> Result<(), RuntimeError> {
        self.condition_pop()?;

        let frame = self.state.function_frames.last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root frame is missing".into()))?;

        let mut branch = match frame.blocks.pop() {
            Some(Block::Branch(branch)) => Ok(branch),
            Some(_) | None => Err(RuntimeError::UnexpectedEndIf),
        }?;

        if branch.then_memory.is_none() {
            branch.then_memory = frame.memory_snapshots.pop();
        } else {
            branch.else_memory = frame.memory_snapshots.pop();
        }

        let mem = frame.memory_snapshots.last_mut()
            .ok_or_else(|| RuntimeError::InternalError("Root block is missing".into()))?;

        match (branch.then_memory, branch.else_memory) {
            (Some(t), Some(f)) => {
                mem.merge(branch.condition, t, f, &mut self.operator)?;
                Ok(())
            }
            (Some(t), None) => {
                let f = mem.fork();
                mem.merge(branch.condition, t, f, &mut self.operator)?;
                Ok(())
            }
            _ => {
                unreachable!()
            }
        }
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
