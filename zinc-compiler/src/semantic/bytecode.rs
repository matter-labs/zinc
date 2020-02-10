//!
//! The Zinc VM bytecode.
//!

use std::collections::HashMap;
use std::ops::Deref;

use zinc_bytecode::data::types::DataType;
use zinc_bytecode::data::types::IntegerType;
use zinc_bytecode::data::types::ScalarType;
use zinc_bytecode::data::values::Value as TemplateValue;
use zinc_bytecode::Instruction;
use zinc_bytecode::Program;

use crate::lexical::Location;
use crate::semantic::element::r#type::Type;

#[derive(Debug, Default, PartialEq)]
pub struct Bytecode {
    input_fields: Vec<(String, Type)>,
    output_type: Type,
    instructions: Vec<Instruction>,

    data_stack_pointer: usize,
    function_addresses: HashMap<usize, usize>,
    address_stack: Vec<usize>,

    current_file: String,
    current_location: Location,
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
            output_type: Type::unit(),
            instructions,

            data_stack_pointer: 0,
            function_addresses,
            address_stack,

            current_file: String::new(),
            current_location: Location::default(),
        }
    }

    pub fn set_main_function(
        &mut self,
        unique_id: usize,
        function_address: usize,
        input_size: usize,
        output_size: usize,
    ) {
        self.instructions[0] =
            Instruction::Call(zinc_bytecode::Call::new(function_address, input_size));
        self.instructions[1] = Instruction::Exit(zinc_bytecode::Exit::new(output_size));
        self.function_addresses.insert(unique_id, function_address);
    }

    pub fn set_input_fields(&mut self, fields: Vec<(String, Type)>) {
        self.input_fields = fields;
    }

    pub fn set_output_type(&mut self, r#type: Type) {
        self.output_type = r#type;
    }

    pub fn push_instruction(&mut self, instruction: Instruction, _location: Location) {
//        if self.current_location != location {
//            if self.current_location.line != location.line {
//                self.instructions
//                    .push(Instruction::LineMarker(zinc_bytecode::LineMarker::new(
//                        location.line,
//                    )));
//            }
//            if self.current_location.column != location.column {
//                self.instructions.push(Instruction::ColumnMarker(
//                    zinc_bytecode::ColumnMarker::new(location.column),
//                ));
//            }
//            self.current_location = location;
//        }
        self.instructions.push(instruction)
    }

    pub fn push_instruction_store(
        &mut self,
        address: usize,
        data_size: usize,
        array_size: Option<usize>,
        is_global: bool,
        location: Location,
    ) {
        self.push_instruction(
            match data_size {
                0 => return,
                1 => match array_size {
                    Some(array_size) => Instruction::StoreByIndex(
                        zinc_bytecode::StoreByIndex::new(address, array_size),
                    ),
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
            },
            location,
        );
    }

    pub fn push_instruction_load(
        &mut self,
        address: usize,
        data_size: usize,
        array_size: Option<usize>,
        is_global: bool,
        location: Location,
    ) {
        self.push_instruction(
            match data_size {
                0 => return,
                1 => match array_size {
                    Some(array_size) if is_global => Instruction::LoadByIndexGlobal(
                        zinc_bytecode::LoadByIndexGlobal::new(address, array_size),
                    ),
                    Some(array_size) => Instruction::LoadByIndex(zinc_bytecode::LoadByIndex::new(
                        address, array_size,
                    )),
                    None if is_global => {
                        Instruction::LoadGlobal(zinc_bytecode::LoadGlobal::new(address))
                    }
                    None => Instruction::Load(zinc_bytecode::Load::new(address)),
                },
                data_size => match array_size {
                    Some(array_size) if is_global => Instruction::LoadSequenceByIndexGlobal(
                        zinc_bytecode::LoadSequenceByIndexGlobal::new(
                            address, array_size, data_size,
                        ),
                    ),
                    Some(array_size) => Instruction::LoadSequenceByIndex(
                        zinc_bytecode::LoadSequenceByIndex::new(address, array_size, data_size),
                    ),
                    None if is_global => Instruction::LoadSequenceGlobal(
                        zinc_bytecode::LoadSequenceGlobal::new(address, data_size),
                    ),
                    None => Instruction::LoadSequence(zinc_bytecode::LoadSequence::new(
                        address, data_size,
                    )),
                },
            },
            location,
        );
    }

    pub fn instructions_len(&self) -> usize {
        self.instructions.len()
    }

    pub fn start_new_file(&mut self, name: &str) {
        self.current_file = name.to_owned();
    }

    pub fn start_new_function(&mut self, _identifier: &str, unique_id: usize) {
        self.function_addresses
            .insert(unique_id, self.instructions.len());
//        self.instructions.push(Instruction::FileMarker(
//            zinc_bytecode::instructions::FileMarker::new(self.current_file.clone()),
//        ));
//        self.instructions.push(Instruction::FunctionMarker(
//            zinc_bytecode::FunctionMarker::new(identifier.to_owned()),
//        ));
        self.data_stack_pointer = 0;
    }

    pub fn function_address(&self, unique_id: usize) -> Option<usize> {
        self.function_addresses.get(&unique_id).copied()
    }

    pub fn allocate_data_stack_space(&mut self, size: usize) -> usize {
        let start_address = self.data_stack_pointer;
        self.data_stack_pointer += size;
        start_address
    }

    pub fn swap_top(&mut self) {
        let last_index = self.instructions.len() - 1;
        let last_but_one_index = self.instructions.len() - 2;
        self.instructions.swap(last_index, last_but_one_index)
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

    pub fn input_template_bytes(&self) -> Vec<u8> {
        let input_type = self.input_types_as_struct();
        let input_template_value = TemplateValue::default_from_type(&input_type);
        match serde_json::to_string_pretty(&input_template_value.to_json()) {
            Ok(json) => json.into_bytes(),
            Err(error) => panic!(
                crate::semantic::PANIC_JSON_TEMPLATE_SERIALIZATION.to_owned()
                    + error.to_string().as_str()
            ),
        }
    }

    pub fn output_template_bytes(&self) -> Vec<u8> {
        let output_bytecode_type = (&self.output_type).into();
        let output_value_template = TemplateValue::default_from_type(&output_bytecode_type);
        match serde_json::to_string_pretty(&output_value_template.to_json()) {
            Ok(json) => (json + "\n").into_bytes(),
            Err(error) => panic!(
                crate::semantic::PANIC_JSON_TEMPLATE_SERIALIZATION.to_owned()
                    + error.to_string().as_str()
            ),
        }
    }

    fn input_types_as_struct(&self) -> DataType {
        DataType::Struct(
            self.input_fields
                .iter()
                .map(|(name, r#type)| (name.clone(), r#type.into()))
                .collect(),
        )
    }
}

impl Into<Vec<Instruction>> for Bytecode {
    fn into(self) -> Vec<Instruction> {
        self.instructions
    }
}

impl Into<Vec<u8>> for Bytecode {
    fn into(self) -> Vec<u8> {
        for (index, instruction) in self.instructions.iter().enumerate() {
            log::debug!("{:03} {:?}", index, instruction)
        }

        let program = Program::new(
            self.input_types_as_struct(),
            (&self.output_type).into(),
            self.instructions,
        );

        program.to_bytes()
    }
}

impl Into<DataType> for &Type {
    fn into(self) -> DataType {
        match self {
            Type::Unit => DataType::Unit,
            Type::Boolean => DataType::Scalar(ScalarType::Boolean),
            Type::IntegerUnsigned { bitlength } => {
                DataType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: false,
                    bit_length: *bitlength,
                }))
            }
            Type::IntegerSigned { bitlength } => {
                DataType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: true,
                    bit_length: *bitlength,
                }))
            }
            Type::Field => DataType::Scalar(ScalarType::Field),
            Type::Enumeration { bitlength, .. } => {
                DataType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: false,
                    bit_length: *bitlength,
                }))
            }
            Type::Array { r#type, size } => {
                let element_type: DataType = r#type.deref().into();
                DataType::Array(Box::new(element_type), *size)
            }
            Type::Tuple { types } => {
                let mut data_types = Vec::new();
                for r#type in types.iter() {
                    data_types.push(r#type.into());
                }
                DataType::Tuple(data_types)
            }
            Type::Structure(structure) => {
                let mut new_fields: Vec<(String, DataType)> = Vec::new();
                for (name, r#type) in structure.fields.iter() {
                    new_fields.push((name.to_owned(), r#type.into()));
                }
                DataType::Struct(new_fields)
            }
            _ => panic!(crate::semantic::PANIC_VALUE_CANNOT_BE_CREATED_FROM),
        }
    }
}
