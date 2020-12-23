//!
//! The Zinc VM generator state.
//!

pub mod entry;
pub mod optimizer;
pub mod unit_test;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_lexical::FILE_INDEX;
use zinc_types::Instruction;

use crate::generator::r#type::contract_field::ContractField as ContractFieldType;
use crate::generator::r#type::Type;
use crate::semantic::analyzer::attribute::Attribute;

use self::entry::Entry;
use self::optimizer::dead_function_code_elimination::Optimizer as DeadFunctionCodeEliminationOptimizer;
use self::unit_test::UnitTest;

///
/// The Zinc VM generator state, used for generating the target code.
///
#[derive(Debug)]
pub struct State {
    /// The Zinc project manifest.
    manifest: zinc_project::Manifest,

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

    /// The application entries hashmap default capacity.
    const ENTRIES_INITIAL_CAPACITY: usize = 16;

    /// The application unit tests hashmap default capacity.
    const UNIT_TESTS_INITIAL_CAPACITY: usize = 16;

    ///
    /// Creates a new bytecode generator state instance.
    ///
    pub fn new(manifest: zinc_project::Manifest) -> Self {
        Self {
            manifest,

            instructions: Vec::with_capacity(Self::INSTRUCTIONS_INITIAL_CAPACITY),
            contract_storage: None,
            entries: HashMap::with_capacity(Self::ENTRIES_INITIAL_CAPACITY),
            unit_tests: HashMap::with_capacity(Self::UNIT_TESTS_INITIAL_CAPACITY),

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
    pub fn unwrap_rc(state: Rc<RefCell<Self>>) -> Self {
        Rc::try_unwrap(state)
            .expect(zinc_const::panic::LAST_SHARED_REFERENCE)
            .into_inner()
    }

    ///
    /// Returns the variable address in the function data stack frame.
    ///
    pub fn get_variable_address(&self, name: &str) -> Option<usize> {
        self.variable_addresses.get(name).copied()
    }

    ///
    /// Sets the contract storage field types.
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
            .push(Instruction::FileMarker(zinc_types::FileMarker::new(
                FILE_INDEX
                    .get_path(location.file)
                    .to_string_lossy()
                    .to_string(),
            )));
        self.instructions.push(Instruction::FunctionMarker(
            zinc_types::FunctionMarker::new(identifier),
        ));

        if let zinc_project::ProjectType::Contract = self.manifest.project.r#type {
            self.define_variable(
                Some(zinc_const::contract::TRANSACTION_VARIABLE_NAME.to_owned()),
                zinc_const::contract::TRANSACTION_SIZE,
            );
        }
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
        attributes: Vec<Attribute>,
    ) {
        let mut should_panic = false;
        let mut is_ignored = false;
        let mut zksync_msg = None;
        for attribute in attributes.into_iter() {
            match attribute {
                Attribute::ShouldPanic => should_panic = true,
                Attribute::Ignore => is_ignored = true,
                Attribute::ZksyncMsg(inner) => zksync_msg = Some(inner),
                _ => {}
            }
        }

        let test = UnitTest::new(
            type_id,
            identifier.clone(),
            should_panic,
            is_ignored,
            zksync_msg,
        );
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
                        .push(Instruction::FileMarker(zinc_types::FileMarker::new(
                            FILE_INDEX
                                .get_path(location.file)
                                .to_string_lossy()
                                .to_string(),
                        )));
                }
                if self.current_location.line != location.line {
                    self.instructions
                        .push(Instruction::LineMarker(zinc_types::LineMarker::new(
                            location.line,
                        )));
                }
                if self.current_location.column != location.column {
                    self.instructions.push(Instruction::ColumnMarker(
                        zinc_types::ColumnMarker::new(location.column),
                    ));
                }
                self.current_location = location;
            }
        }

        self.instructions.push(instruction)
    }

    ///
    /// Converts the compiled application state into a set of byte arrays, which are ready to be
    /// written to the Zinc project build files.
    ///
    pub fn into_application(
        mut self,
        optimize_dead_function_elimination: bool,
    ) -> zinc_types::Application {
        match self.contract_storage.take() {
            Some(storage) => {
                let storage = storage.into_iter().map(|field| field.into()).collect();

                if optimize_dead_function_elimination {
                    let mut entry_ids: Vec<usize> = self
                        .entries
                        .iter()
                        .map(|(_name, method)| method.type_id)
                        .collect();
                    entry_ids.extend(
                        self.unit_tests
                            .iter()
                            .map(|(_name, unit_test)| unit_test.type_id)
                            .collect::<Vec<usize>>(),
                    );

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
                    let mut input: zinc_types::Type = method.input_fields_as_struct().into();
                    input.set_contract_address();
                    let output = method.output_type.into();
                    methods.insert(
                        method.name.clone(),
                        zinc_types::ContractMethod::new(
                            type_id,
                            method.name,
                            address,
                            method.is_mutable,
                            input,
                            output,
                        ),
                    );
                }

                let mut unit_tests = HashMap::with_capacity(self.unit_tests.len());
                for (type_id, unit_test) in self.unit_tests.into_iter() {
                    let address = self
                        .function_addresses
                        .get(&type_id)
                        .cloned()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                    unit_tests.insert(
                        unit_test.name,
                        zinc_types::UnitTest::new(
                            address,
                            unit_test.should_panic,
                            unit_test.is_ignored,
                            unit_test.zksync_msg,
                        ),
                    );
                }

                Self::print_instructions(self.instructions.as_slice());

                zinc_types::Application::new_contract(
                    self.manifest.project.name,
                    storage,
                    methods,
                    unit_tests,
                    self.instructions,
                )
            }
            None if !self.entries.is_empty() => {
                let (entry_id, entry) = self
                    .entries
                    .into_iter()
                    .collect::<Vec<(usize, Entry)>>()
                    .remove(0);
                let input = entry.input_fields_as_struct().into();
                let output = entry.output_type.into();

                if optimize_dead_function_elimination {
                    let mut entry_ids: Vec<usize> = vec![entry_id];
                    entry_ids.extend(
                        self.unit_tests
                            .iter()
                            .map(|(_name, unit_test)| unit_test.type_id)
                            .collect::<Vec<usize>>(),
                    );

                    DeadFunctionCodeEliminationOptimizer::optimize(
                        entry_ids,
                        &mut self.instructions,
                        &mut self.function_addresses,
                    );
                } else {
                    DeadFunctionCodeEliminationOptimizer::set_addresses(
                        &mut self.instructions,
                        &self.function_addresses,
                    );
                }

                let mut unit_tests = HashMap::with_capacity(self.unit_tests.len());
                for (type_id, unit_test) in self.unit_tests.into_iter() {
                    let address = self
                        .function_addresses
                        .get(&type_id)
                        .cloned()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                    unit_tests.insert(
                        unit_test.name,
                        zinc_types::UnitTest::new(
                            address,
                            unit_test.should_panic,
                            unit_test.is_ignored,
                            unit_test.zksync_msg,
                        ),
                    );
                }

                let address = self
                    .function_addresses
                    .get(&entry_id)
                    .cloned()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

                Self::print_instructions(self.instructions.as_slice());

                zinc_types::Application::new_circuit(
                    self.manifest.project.name,
                    address,
                    input,
                    output,
                    unit_tests,
                    self.instructions,
                )
            }
            None => {
                DeadFunctionCodeEliminationOptimizer::set_addresses(
                    &mut self.instructions,
                    &self.function_addresses,
                );

                let mut unit_tests = HashMap::with_capacity(self.unit_tests.len());
                for (type_id, unit_test) in self.unit_tests.into_iter() {
                    let address = self
                        .function_addresses
                        .get(&type_id)
                        .cloned()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                    unit_tests.insert(
                        unit_test.name,
                        zinc_types::UnitTest::new(
                            address,
                            unit_test.should_panic,
                            unit_test.is_ignored,
                            unit_test.zksync_msg,
                        ),
                    );
                }

                Self::print_instructions(self.instructions.as_slice());

                zinc_types::Application::new_library(
                    self.manifest.project.name,
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
