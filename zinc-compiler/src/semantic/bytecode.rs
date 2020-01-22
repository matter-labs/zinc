//!
//! The Zinc VM bytecode.
//!

use std::collections::HashMap;

use zinc_bytecode::{Program, Instruction};
use zinc_bytecode::data::types::*;

use crate::semantic::Type;

#[derive(Debug, Default, PartialEq)]
pub struct Bytecode {
    input_fields: Vec<(String, Type)>,
    output_type: Type,
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
            input_fields: vec![],
            output_type: Type::new_unit(),
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

    pub fn set_input_fields(&mut self, fields: Vec<(String, Type)>) {
        self.input_fields = fields;
    }

    pub fn set_output_type(&mut self, r#type: Type) {
        self.output_type = r#type;
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

    pub fn input_template_bytes(&self) -> Vec<u8> {
        let input_type = Type::new_structure("input".to_owned(), self.input_fields.clone(), None);
        match input_type.to_json_template() {
            Some(input) => input.to_string().into_bytes(),
            None => vec![],
        }
    }

    pub fn output_template_bytes(&self) -> Vec<u8> {
        match self.output_type.to_json_template() {
            Some(output) => output.to_string().into_bytes(),
            None => vec![],
        }
    }
}

impl Into<Vec<Instruction>> for Bytecode {
    fn into(self) -> Vec<Instruction> {
        self.instructions
    }
}

impl Into<Vec<u8>> for Bytecode {
    fn into(self) -> Vec<u8> {
        for instruction in self.instructions.iter() {
            log::trace!("{:?}", instruction)
        }

        let program = Program::new(
            self.input_fields
                .into_iter()
                .map(|(name, r#type)| (name, r#type.into()))
                .collect(),
            self.output_type.into(),
            self.instructions,
        );

        program.to_bytes()
    }
}

impl Into<DataType> for Type {
    fn into(self) -> DataType {
        match self {
            Type::Unit => DataType::Unit,
            Type::Boolean => DataType::Primitive(PrimitiveType::Integer(BinaryInteger {
                is_signed: false,
                bit_length: crate::BITLENGTH_BOOLEAN,
            })),
            Type::IntegerUnsigned { bitlength } => {
                DataType::Primitive(PrimitiveType::Integer(BinaryInteger {
                    is_signed: false,
                    bit_length: bitlength,
                }))
            }
            Type::IntegerSigned { bitlength } => {
                DataType::Primitive(PrimitiveType::Integer(BinaryInteger {
                    is_signed: true,
                    bit_length: bitlength,
                }))
            }
            Type::Field => DataType::Primitive(PrimitiveType::Field),
            Type::Enumeration { bitlength, .. } => {
                DataType::Primitive(PrimitiveType::Integer(BinaryInteger {
                    is_signed: false,
                    bit_length: bitlength,
                }))
            }
            Type::Array { r#type, size } => {
                let element_type: DataType = (*r#type).into();
                DataType::Array(Box::new(element_type), size)
            }
            Type::Tuple { types } => {
                let mut data_types = Vec::new();
                for r#type in types.into_iter() {
                    data_types.push(r#type.into());
                }
                DataType::Tuple(data_types)
            }
            Type::Structure { fields, .. } => {
                let mut new_fields: Vec<(String, DataType)> = Vec::new();
                for (name, r#type) in fields.into_iter() {
                    new_fields.push(
                        (name, r#type.into())
                    );
                }
                DataType::Struct(new_fields)
            }
            _ => panic!(crate::semantic::PANIC_VALUE_CANNOT_BE_CREATED_FROM),
        }
    }
}
