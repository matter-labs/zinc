//!
//! The Zinc VM bytecode.
//!

use std::collections::HashMap;

use zinc_bytecode::dispatch_instruction;
use zinc_bytecode::Instruction;
use zinc_bytecode::InstructionInfo;

#[derive(Debug, Default, PartialEq)]
pub struct Bytecode {
    instructions: Vec<Instruction>,
    stack_pointer: usize,
    function_addresses: HashMap<String, usize>,
    call_stack: Vec<usize>,
}

impl Bytecode {
    const INSTRUCTION_VECTOR_INITIAL_SIZE: usize = 1024;
    const FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE: usize = 16;
    const CALL_STACK_VECTOR_INITIAL_SIZE: usize = 16;

    pub fn new_binary() -> Self {
        let mut instructions = Vec::with_capacity(Self::INSTRUCTION_VECTOR_INITIAL_SIZE);
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));
        let function_addresses =
            HashMap::with_capacity(Self::FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE);
        let call_stack = Vec::with_capacity(Self::CALL_STACK_VECTOR_INITIAL_SIZE);

        Self {
            instructions,
            stack_pointer: 0,
            function_addresses,
            call_stack,
        }
    }

    pub fn new_library() -> Self {
        let instructions = Vec::with_capacity(Self::INSTRUCTION_VECTOR_INITIAL_SIZE);
        let function_addresses =
            HashMap::with_capacity(Self::FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE);
        let call_stack = Vec::with_capacity(Self::CALL_STACK_VECTOR_INITIAL_SIZE);

        Self {
            instructions,
            stack_pointer: 0,
            function_addresses,
            call_stack,
        }
    }

    pub fn set_main_function(
        &mut self,
        function_address: usize,
        input_size: usize,
        output_size: usize,
    ) {
        self.instructions[0] =
            Instruction::Call(zinc_bytecode::Call::new(function_address, input_size));
        self.instructions[1] = Instruction::Exit(zinc_bytecode::Exit::new(output_size));
        self.function_addresses
            .insert("main".to_owned(), function_address);
    }

    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub fn push_instruction_pop_store(&mut self, address: usize, size: usize) {
        match size {
            0 => {}
            1 => self
                .instructions
                .push(Instruction::PopStore(zinc_bytecode::PopStore::new(address))),
            size => self.instructions.push(Instruction::PopStoreArray(
                zinc_bytecode::PopStoreArray::new(address, size),
            )),
        }
    }

    pub fn push_instruction_load_push(&mut self, address: usize, size: usize) {
        match size {
            0 => {}
            1 => self
                .instructions
                .push(Instruction::LoadPush(zinc_bytecode::LoadPush::new(address))),
            size => self.instructions.push(Instruction::LoadPushArray(
                zinc_bytecode::LoadPushArray::new(address, size),
            )),
        }
    }

    pub fn start_new_function(&mut self, identifier: &str) {
        self.function_addresses
            .insert(identifier.to_owned(), self.instructions.len());
        self.stack_pointer = 0;
    }

    pub fn function_address(&self, identifier: &str) -> Option<usize> {
        self.function_addresses.get(identifier).copied()
    }

    pub fn allocate_stack_space(&mut self, size: usize) -> usize {
        let start_address = self.stack_pointer;
        self.stack_pointer += size;
        start_address
    }

    pub fn push_call_stack_pointer(&mut self) {
        self.call_stack.push(self.stack_pointer);
    }

    pub fn pop_call_stack_pointer(&mut self) {
        self.stack_pointer = self
            .call_stack
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_CALL_STACK_POINTER);
    }

    pub fn into_instructions(self) -> Vec<Instruction> {
        self.instructions
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.instructions
            .into_iter()
            .enumerate()
            .map(|(index, instruction)| {
                log::trace!("{:03} {:?}", index, instruction);
                dispatch_instruction!(instruction => instruction.encode())
            })
            .flatten()
            .collect::<Vec<u8>>()
    }
}
