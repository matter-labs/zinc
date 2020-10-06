//!
//! The Zinc VM generator state.
//!

pub mod entry;
pub mod optimizer;
pub mod unit_test;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_build::Application as BuildApplication;
use zinc_build::ContractFieldType as BuildContractFieldType;
use zinc_build::ContractMethod;
use zinc_build::Instruction;
use zinc_build::Type as BuildType;
use zinc_build::UnitTest as BuildUnitTest;

use crate::generator::r#type::contract_field::ContractField as ContractFieldType;
use crate::generator::r#type::Type;
use crate::lexical::token::location::Location;
use crate::source::file::index::INDEX as FILE_INDEX;

use self::entry::Entry;
use self::optimizer::dead_function_code_elimination::Optimizer as DeadFunctionCodeEliminationOptimizer;
use self::unit_test::UnitTest;

///
/// The Zinc VM generator state, used for generating the target code.
///
#[derive(Debug)]
pub struct State {
    /// The Zinc application name.
    name: String,

    /// The Zinc VM instructions written by the bytecode generator.
    instructions: Vec<Instruction>,
    /// The contract storage structure.
    contract_storage: Option<Vec<ContractFieldType>>,
    /// Metadata of each application entry.
    entries: HashMap<usize, Entry>,
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

    /// The contract method hashmap default capacity.
    const METHODS_INITIAL_CAPACITY: usize = 16;

    ///
    /// Creates a new bytecode instance with the placeholders for the entry `Call` and
    /// `Exit` instructions.
    ///
    pub fn new(name: String) -> Self {
        Self {
            name,

            instructions: Vec::with_capacity(Self::INSTRUCTIONS_INITIAL_CAPACITY),
            contract_storage: None,
            entries: HashMap::with_capacity(Self::METHODS_INITIAL_CAPACITY),
            unit_tests: HashMap::with_capacity(Self::METHODS_INITIAL_CAPACITY),

            function_addresses: HashMap::with_capacity(Self::FUNCTION_ADDRESSES_INITIAL_CAPACITY),
            variable_addresses: HashMap::with_capacity(Self::VARIABLE_ADDRESSES_INITIAL_CAPACITY),
            data_stack_pointer: 0,
            current_location: Location::default(),
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
    /// Returns the application name.
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
    pub fn set_contract_storage(&mut self, fields: Vec<ContractFieldType>) {
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

        self.instructions
            .push(Instruction::FileMarker(zinc_build::FileMarker::new(
                FILE_INDEX
                    .get_path(location.file)
                    .to_string_lossy()
                    .to_string(),
            )));
        self.instructions.push(Instruction::FunctionMarker(
            zinc_build::FunctionMarker::new(identifier),
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
        input_arguments: Vec<(String, bool, Type)>,
        output_type: Type,
    ) {
        let method = Entry::new(
            type_id,
            identifier.clone(),
            is_mutable,
            input_arguments,
            output_type,
        );
        self.entries.insert(type_id, method);

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
                if self.instructions.is_empty() || self.current_location.file != location.file {
                    self.instructions
                        .push(Instruction::FileMarker(zinc_build::FileMarker::new(
                            FILE_INDEX
                                .get_path(location.file)
                                .to_string_lossy()
                                .to_string(),
                        )));
                }
                if self.current_location.line != location.line {
                    self.instructions
                        .push(Instruction::LineMarker(zinc_build::LineMarker::new(
                            location.line,
                        )));
                }
                if self.current_location.column != location.column {
                    self.instructions.push(Instruction::ColumnMarker(
                        zinc_build::ColumnMarker::new(location.column),
                    ));
                }
                self.current_location = location;
            }
        }

        self.instructions.push(instruction)
    }

    ///
    /// Returns the contract storage structure if the application is a contract.
    ///
    /// Converts the generator types to the VM ones.
    ///
    pub fn contract_storage(&self) -> Option<Vec<BuildContractFieldType>> {
        self.contract_storage
            .to_owned()
            .map(|storage| storage.into_iter().map(|field| field.into()).collect())
    }

    ///
    /// Converts the compiled application state into a set of byte arrays, which are ready to be
    /// written to the Zinc project build files.
    ///
    pub fn into_application(
        mut self,
        optimize_dead_function_elimination: bool,
    ) -> BuildApplication {
        match self.contract_storage.take() {
            Some(storage) => {
                let mut unit_tests = HashMap::with_capacity(self.unit_tests.len());
                for (type_id, unit_test) in self.unit_tests.into_iter() {
                    let address = self
                        .function_addresses
                        .get(&type_id)
                        .cloned()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                    unit_tests.insert(
                        unit_test.name,
                        BuildUnitTest::new(address, unit_test.should_panic, unit_test.is_ignored),
                    );
                }

                let storage = storage.into_iter().map(|field| field.into()).collect();

                if optimize_dead_function_elimination {
                    let entry_ids: Vec<usize> = self
                        .entries
                        .iter()
                        .map(|(_name, method)| method.type_id)
                        .collect();
                    DeadFunctionCodeEliminationOptimizer::optimize(
                        entry_ids,
                        &mut self.instructions,
                        &mut self.function_addresses,
                    );
                } else {
                    DeadFunctionCodeEliminationOptimizer::set_addresses(
                        &mut self.instructions,
                        &self.function_addresses,
                    )
                }

                let mut methods = HashMap::with_capacity(self.entries.len());
                for (type_id, method) in self.entries.into_iter() {
                    let address = self
                        .function_addresses
                        .get(&type_id)
                        .cloned()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                    let mut input: BuildType = method.input_fields_as_struct().into();
                    input.remove_contract_instance();
                    let output = method.output_type.into();
                    methods.insert(
                        method.name,
                        ContractMethod::new(type_id, address, method.is_mutable, input, output),
                    );
                }

                Self::print_instructions(self.instructions.as_slice());

                BuildApplication::new_contract(
                    self.name,
                    storage,
                    methods,
                    unit_tests,
                    self.instructions,
                )
            }
            None => {
                let mut unit_tests = HashMap::with_capacity(self.unit_tests.len());
                for (type_id, unit_test) in self.unit_tests.into_iter() {
                    let address = self
                        .function_addresses
                        .get(&type_id)
                        .cloned()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                    unit_tests.insert(
                        unit_test.name,
                        BuildUnitTest::new(address, unit_test.should_panic, unit_test.is_ignored),
                    );
                }

                let (entry_id, entry) = self
                    .entries
                    .into_iter()
                    .collect::<Vec<(usize, Entry)>>()
                    .remove(0);
                let input = entry.input_fields_as_struct().into();
                let output = entry.output_type.into();

                if optimize_dead_function_elimination {
                    DeadFunctionCodeEliminationOptimizer::optimize(
                        vec![entry_id],
                        &mut self.instructions,
                        &mut self.function_addresses,
                    );
                } else {
                    DeadFunctionCodeEliminationOptimizer::set_addresses(
                        &mut self.instructions,
                        &self.function_addresses,
                    )
                }

                let address = self
                    .function_addresses
                    .get(&entry_id)
                    .cloned()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

                Self::print_instructions(self.instructions.as_slice());

                BuildApplication::new_circuit(
                    self.name,
                    address,
                    input,
                    output,
                    unit_tests,
                    self.instructions,
                )
            }
        }
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
