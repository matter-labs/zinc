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
    data_stack_pointer: usize,
    function_addresses: HashMap<String, usize>,
    address_stack: Vec<usize>,
}

impl Bytecode {
    const INSTRUCTION_VECTOR_INITIAL_SIZE: usize = 1024;
    const FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE: usize = 16;
    const ADDRESS_STACK_VECTOR_INITIAL_SIZE: usize = 16;

    pub fn new() -> Self {
        let mut instructions = Vec::with_capacity(Self::INSTRUCTION_VECTOR_INITIAL_SIZE);
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));
        let function_addresses =
            HashMap::with_capacity(Self::FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE);
        let address_stack = Vec::with_capacity(Self::ADDRESS_STACK_VECTOR_INITIAL_SIZE);

        Self {
            instructions,
            data_stack_pointer: 0,
            function_addresses,
            address_stack,
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

    pub fn push_instruction_store(
        &mut self,
        address: usize,
        data_size: usize,
        array_size: Option<usize>,
    ) {
        self.instructions.push(match data_size {
            0 => return,
            1 => match array_size {
                Some(array_size) => {
                    Instruction::StoreByIndex(zinc_bytecode::StoreByIndex::new(address, array_size))
                }
                None => Instruction::Store(zinc_bytecode::Store::new(address)),
            },
            data_size => match array_size {
                Some(array_size) => Instruction::StoreArrayByIndex(
                    zinc_bytecode::StoreArrayByIndex::new(address, array_size, data_size),
                ),
                None => Instruction::StoreArray(zinc_bytecode::StoreArray::new(address, data_size)),
            },
        });
    }

    pub fn push_instruction_load(
        &mut self,
        address: usize,
        data_size: usize,
        array_size: Option<usize>,
        is_global: bool,
    ) {
        self.instructions.push(match data_size {
            0 => return,
            1 => match array_size {
                Some(array_size) if is_global => Instruction::LoadByIndexGlobal(
                    zinc_bytecode::LoadByIndexGlobal::new(address, array_size),
                ),
                Some(array_size) => {
                    Instruction::LoadByIndex(zinc_bytecode::LoadByIndex::new(address, array_size))
                }
                None if is_global => {
                    Instruction::LoadGlobal(zinc_bytecode::LoadGlobal::new(address))
                }
                None => Instruction::Load(zinc_bytecode::Load::new(address)),
            },
            data_size => match array_size {
                Some(array_size) if is_global => Instruction::LoadArrayByIndexGlobal(
                    zinc_bytecode::LoadArrayByIndexGlobal::new(address, array_size, data_size),
                ),
                Some(array_size) => Instruction::LoadArrayByIndex(
                    zinc_bytecode::LoadArrayByIndex::new(address, array_size, data_size),
                ),
                None if is_global => Instruction::LoadArrayGlobal(
                    zinc_bytecode::LoadArrayGlobal::new(address, data_size),
                ),
                None => Instruction::LoadArray(zinc_bytecode::LoadArray::new(address, data_size)),
            },
        });
    }

    pub fn start_new_function(&mut self, identifier: &str) {
        self.function_addresses
            .insert(identifier.to_owned(), self.instructions.len());
        self.data_stack_pointer = 0;
    }

    pub fn function_address(&self, identifier: &str) -> Option<usize> {
        self.function_addresses.get(identifier).copied()
    }

    pub fn allocate_data_stack_space(&mut self, size: usize) -> usize {
        let start_address = self.data_stack_pointer;
        self.data_stack_pointer += size;
        start_address
    }

    pub fn push_data_stack_address(&mut self) {
        self.address_stack.push(self.data_stack_pointer);
    }

    pub fn pop_data_stack_address(&mut self) {
        self.data_stack_pointer = self
            .address_stack
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
