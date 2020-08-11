//!
//! The Zinc VM generator state.
//!

pub mod bytecode;
pub mod method;
pub mod optimizer;
pub mod unit_test;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_bytecode::CircuitUnitTest;
use zinc_bytecode::ContractMethod;
use zinc_bytecode::ContractUnitTest;
use zinc_bytecode::DataType;
use zinc_bytecode::Instruction;
use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;

use crate::generator::r#type::Type;
use crate::lexical::token::location::Location;
use crate::source::file::index::INDEX as FILE_INDEX;

use self::bytecode::Bytecode;
use self::method::Method;
use self::optimizer::elimination::Optimizer as EliminationOptimizer;
use self::unit_test::UnitTest;

///
/// The Zinc VM generator state, used for generating the target code.
///
#[derive(Debug)]
pub struct State {
    /// The Zinc program name.
    name: String,

    /// The Zinc VM instructions written by the bytecode generator.
    instructions: Vec<Instruction>,
    /// The contract storage structure.
    contract_storage: Option<Vec<(String, Type)>>,
    /// Metadata of each contract method.
    methods: HashMap<usize, Method>,
    /// Unit tests.
    unit_tests: HashMap<usize, UnitTest>,

    /// Bytecode addresses of the functions written to the bytecode.
    function_addresses: HashMap<usize, usize>,
    /// Data stack addresses of variables declared at runtime.
    variable_addresses: HashMap<String, usize>,
    /// The pointer which is reset at the beginning of each function.
    data_stack_pointer: usize,
    /// The location pointer used to pass debug information to the VM.
    current_location: Location,
}

impl State {
    /// The instruction array default capacity.
    const INSTRUCTIONS_INITIAL_CAPACITY: usize = 1024;

    /// The function address hashmap default capacity.
    const FUNCTION_ADDRESSES_INITIAL_CAPACITY: usize = 16;

    /// The variable address hashmap default capacity.
    const VARIABLE_ADDRESSES_INITIAL_CAPACITY: usize = 16;

    /// The application entry hashmap default capacity.
    const ENTRY_METADATA_INITIAL_CAPACITY: usize = 16;

    ///
    /// Creates a new bytecode instance with the placeholders for the entry `Call` and
    /// `Exit` instructions.
    ///
    pub fn new(name: String) -> Self {
        Self {
            name,

            instructions: Vec::with_capacity(Self::INSTRUCTIONS_INITIAL_CAPACITY),
            contract_storage: None,
            methods: HashMap::with_capacity(Self::ENTRY_METADATA_INITIAL_CAPACITY),
            unit_tests: HashMap::with_capacity(Self::ENTRY_METADATA_INITIAL_CAPACITY),

            function_addresses: HashMap::with_capacity(Self::FUNCTION_ADDRESSES_INITIAL_CAPACITY),
            variable_addresses: HashMap::with_capacity(Self::VARIABLE_ADDRESSES_INITIAL_CAPACITY),
            data_stack_pointer: 0,
            current_location: Location::new_beginning(None),
        }
    }

    ///
    /// Wraps the bytecode into `Rc<RefCell<_>>` simplifying most of initializations.
    ///
    pub fn wrap(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    ///
    /// Extracts the bytecode from `Rc<RefCell<_>>`.
    ///
    pub fn unwrap_rc(bytecode: Rc<RefCell<Self>>) -> Self {
        Rc::try_unwrap(bytecode)
            .expect(zinc_const::panic::LAST_SHARED_REFERENCE)
            .into_inner()
    }

    ///
    /// Returns the program name.
    ///
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Returns the variable address in the function data stack frame.
    ///
    pub fn get_variable_address(&self, name: &str) -> Option<usize> {
        self.variable_addresses.get(name).copied()
    }

    ///
    /// Sets the contract storage, which means the application is a contract.
    ///
    pub fn set_contract_storage(&mut self, fields: Vec<(String, Type)>) {
        self.contract_storage = Some(fields);
    }

    ///
    /// Starts a new function, resetting the data stack pointer and writing the
    /// function debug information.
    ///
    pub fn start_function(&mut self, location: Location, type_id: usize, identifier: String) {
        let address = self.instructions.len();
        self.function_addresses.insert(type_id, address);
        self.data_stack_pointer = 0;

        let file_path = FILE_INDEX
            .get_path(location.file_index)
            .to_string_lossy()
            .to_string();
        self.instructions
            .push(Instruction::FileMarker(zinc_bytecode::FileMarker::new(
                file_path,
            )));
        self.instructions.push(Instruction::FunctionMarker(
            zinc_bytecode::FunctionMarker::new(identifier),
        ));
    }

    ///
    /// Starts an entry function, saves its metadata and calls the `start_function` method.
    ///
    pub fn start_entry_function(
        &mut self,
        location: Location,
        type_id: usize,
        identifier: String,
        is_mutable: bool,
        input_arguments: Vec<(String, Type)>,
        output_type: Type,
    ) {
        let method = Method::new(identifier.clone(), is_mutable, input_arguments, output_type);
        self.methods.insert(type_id, method);

        self.start_function(location, type_id, identifier);
    }

    ///
    /// Starts a unit test function, saves its metadata and calls the `start_function` method.
    ///
    pub fn start_unit_test_function(
        &mut self,
        location: Location,
        type_id: usize,
        identifier: String,
        should_panic: bool,
        is_ignored: bool,
    ) {
        let test = UnitTest::new(identifier.clone(), should_panic, is_ignored);
        self.unit_tests.insert(type_id, test);

        self.start_function(location, type_id, identifier);
    }

    ///
    /// Defines a variable, saving its address within the current data stack frame.
    ///
    pub fn define_variable(&mut self, identifier: Option<String>, size: usize) -> usize {
        let start_address = self.data_stack_pointer;
        if let Some(identifier) = identifier {
            self.variable_addresses
                .insert(identifier, self.data_stack_pointer);
        }
        self.data_stack_pointer += size;
        start_address
    }

    ///
    /// Writes the instruction along with its location debug information.
    ///
    pub fn push_instruction(&mut self, instruction: Instruction, location: Option<Location>) {
        if let Some(location) = location {
            if self.current_location != location {
                if self.current_location.line != location.line {
                    self.instructions.push(Instruction::LineMarker(
                        zinc_bytecode::LineMarker::new(location.line),
                    ));
                }
                if self.current_location.column != location.column {
                    self.instructions.push(Instruction::ColumnMarker(
                        zinc_bytecode::ColumnMarker::new(location.column),
                    ));
                }
                self.current_location = location;
            }
        }

        self.instructions.push(instruction)
    }

    ///
    /// Returns the contract storage structure if the program is a contract.
    ///
    /// Converts the generator types to the VM ones.
    ///
    pub fn contract_storage(&self) -> Option<Vec<(String, DataType)>> {
        self.contract_storage.to_owned().map(|storage| {
            storage
                .into_iter()
                .map(|(name, r#type)| (name, r#type.into()))
                .collect()
        })
    }

    ///
    /// Generates the bytecode for the entry specified with `entry_id`.
    ///
    /// Besides that, the function walks through the `Call` instructions and replaces `type_id`s
    /// of the function stored as `address`es with their actual addresses in the bytecode,
    /// since the addresses are only known after the functions are written thereto.
    ///
    pub fn into_program(mut self, optimize_dead_function_elimination: bool) -> BytecodeProgram {
        Self::print_instructions(self.instructions.as_slice());

        match self.contract_storage.take() {
            Some(storage) => {
                let mut unit_tests = HashMap::with_capacity(self.unit_tests.len());
                for (unique_id, unit_test) in self.unit_tests.into_iter() {
                    let address = self.function_addresses.get(&unique_id).cloned().unwrap();
                    unit_tests.insert(
                        unit_test.name,
                        ContractUnitTest::new(
                            address,
                            unit_test.should_panic,
                            unit_test.is_ignored,
                        ),
                    );
                }

                let mut methods = HashMap::with_capacity(self.methods.len());
                for (unique_id, method) in self.methods.into_iter() {
                    let address = self.function_addresses.get(&unique_id).cloned().unwrap();
                    let input = method.input_fields_as_struct().into();
                    let output = method.output_type.into();
                    methods.insert(method.name, ContractMethod::new(input, output, address));
                }

                let storage = storage
                    .into_iter()
                    .map(|(name, r#type)| (name, r#type.into()))
                    .collect();

                BytecodeProgram::new_contract(
                    self.name,
                    self.instructions,
                    storage,
                    methods,
                    unit_tests,
                )
            }
            None => {
                let mut unit_tests = HashMap::with_capacity(self.unit_tests.len());
                for (unique_id, unit_test) in self.unit_tests.into_iter() {
                    let address = self.function_addresses.get(&unique_id).cloned().unwrap();
                    unit_tests.insert(
                        unit_test.name,
                        CircuitUnitTest::new(address, unit_test.should_panic, unit_test.is_ignored),
                    );
                }

                let entry = self
                    .methods
                    .into_iter()
                    .map(|(_name, entry)| entry)
                    .collect::<Vec<Method>>()
                    .remove(0);

                let input = entry.input_fields_as_struct().into();
                let output = entry.output_type.into();
                BytecodeProgram::new_circuit(
                    self.name,
                    self.instructions,
                    input,
                    output,
                    unit_tests,
                )
            }
        }
        //            if optimize_dead_function_elimination {
        //                EliminationOptimizer::optimize(
        //                    entry_id,
        //                    &mut instructions,
        //                    self.function_addresses.clone(),
        //                );
        //            } else {
        //                EliminationOptimizer::set_addresses(&mut instructions, &self.function_addresses);
        //            };
    }

    ///
    /// Prints the bytecode instructions to the terminal.
    ///
    /// Some instructions, like location markers, are only printed on `trace` logging level.
    ///
    fn print_instructions(instructions: &[Instruction]) {
        for (index, instruction) in instructions.iter().enumerate() {
            if instruction.is_debug() {
                log::trace!("{:03} {:?}", index, instruction)
            } else {
                log::debug!("{:03} {:?}", index, instruction)
            }
        }
    }
}
