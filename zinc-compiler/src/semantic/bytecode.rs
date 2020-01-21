//!
//! The Zinc VM bytecode.
//!

use std::collections::HashMap;
use std::mem;

use zinc_bytecode::dispatch_instruction;
use zinc_bytecode::Instruction;
use zinc_bytecode::InstructionInfo;

use crate::semantic::Type;

const VERSION: u64 = 0x0000_0000_0001_0000;

#[derive(Debug, Default, PartialEq)]
pub struct Bytecode {
    input_type: Type,
    witness_type: Type,
    result_type: Type,
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
            input_type: Type::new_unit(),
            witness_type: Type::new_unit(),
            result_type: Type::new_unit(),
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

    pub fn set_input_type(&mut self, r#type: Type) {
        self.input_type = r#type;
    }

    pub fn set_witness_type(&mut self, r#type: Type) {
        self.witness_type = r#type;
    }

    pub fn set_result_type(&mut self, r#type: Type) {
        self.result_type = r#type;
    }

    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }

    pub fn insert_instruction(&mut self, index: usize, instruction: Instruction) {
        self.instructions.insert(index, instruction)
    }

    pub fn push_instruction_store(
        &mut self,
        address: usize,
        data_size: usize,
        array_size: Option<usize>,
        is_global: bool,
    ) {
        self.instructions.push(match data_size {
            0 => return,
            1 => match array_size {
                Some(array_size) => {
                    Instruction::StoreByIndex(zinc_bytecode::StoreByIndex::new(address, array_size))
                }
                None if is_global => {
                    Instruction::StoreGlobal(zinc_bytecode::StoreGlobal::new(address))
                }
                None => Instruction::Store(zinc_bytecode::Store::new(address)),
            },
            data_size => match array_size {
                Some(array_size) => Instruction::StoreSequenceByIndex(
                    zinc_bytecode::StoreSequenceByIndex::new(address, array_size, data_size),
                ),
                None if is_global => Instruction::StoreSequenceGlobal(
                    zinc_bytecode::StoreSequenceGlobal::new(address, data_size),
                ),
                None => Instruction::StoreSequence(zinc_bytecode::StoreSequence::new(
                    address, data_size,
                )),
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
                Some(array_size) if is_global => Instruction::LoadSequenceByIndexGlobal(
                    zinc_bytecode::LoadSequenceByIndexGlobal::new(address, array_size, data_size),
                ),
                Some(array_size) => Instruction::LoadSequenceByIndex(
                    zinc_bytecode::LoadSequenceByIndex::new(address, array_size, data_size),
                ),
                None if is_global => Instruction::LoadSequenceGlobal(
                    zinc_bytecode::LoadSequenceGlobal::new(address, data_size),
                ),
                None => {
                    Instruction::LoadSequence(zinc_bytecode::LoadSequence::new(address, data_size))
                }
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

    pub fn next_position(&self) -> usize {
        self.instructions.len()
    }

    pub fn into_instructions(self) -> Vec<Instruction> {
        self.instructions
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let input_metadata = match self.input_type.to_json_metadata() {
            Some(input) => {
                let input = input.to_string().into_bytes();
                let mut data = Vec::with_capacity(mem::size_of::<u64>() + input.len());
                data.extend(input.len().to_be_bytes().to_vec());
                data.extend(input);
                data
            }
            None => vec![0u8; mem::size_of::<u64>()],
        };
        let witness_metadata = match self.witness_type.to_json_metadata() {
            Some(witness) => {
                let witness = witness.to_string().into_bytes();
                let mut data = Vec::with_capacity(mem::size_of::<u64>() + witness.len());
                data.extend(witness.len().to_be_bytes().to_vec());
                data.extend(witness);
                data
            }
            None => vec![0u8; mem::size_of::<u64>()],
        };
        let instruction_data = self
            .instructions
            .into_iter()
            .enumerate()
            .map(|(index, instruction)| {
                log::trace!("{:03} {:?}", index, instruction);
                dispatch_instruction!(instruction => instruction.encode())
            })
            .flatten()
            .collect::<Vec<u8>>();

        let mut result = Vec::with_capacity(
            mem::size_of_val(&VERSION)
                + input_metadata.len()
                + witness_metadata.len()
                + instruction_data.len(),
        );
        result.extend(VERSION.to_be_bytes().to_vec());
        result.extend(input_metadata);
        result.extend(witness_metadata);
        result.extend(instruction_data);
        result
    }

    pub fn input_template_bytes(&self) -> Vec<u8> {
        match self.input_type.to_json_template() {
            Some(input) => serde_json::to_string_pretty(&input)
                .expect(crate::semantic::PANIC_PRETTY_SERIALIZATION_INVALID)
                .into_bytes(),
            None => vec![],
        }
    }

    pub fn witness_template_bytes(&self) -> Vec<u8> {
        match self.witness_type.to_json_template() {
            Some(witness) => serde_json::to_string_pretty(&witness)
                .expect(crate::semantic::PANIC_PRETTY_SERIALIZATION_INVALID)
                .into_bytes(),
            None => vec![],
        }
    }

    pub fn result_template_bytes(&self) -> Vec<u8> {
        match self.result_type.to_json_template() {
            Some(result) => serde_json::to_string_pretty(&result)
                .expect(crate::semantic::PANIC_PRETTY_SERIALIZATION_INVALID)
                .into_bytes(),
            None => vec![],
        }
    }
}
