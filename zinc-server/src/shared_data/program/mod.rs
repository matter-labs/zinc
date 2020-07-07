//!
//! The program model.
//!

pub mod entry;

use std::collections::HashMap;

use zinc_bytecode::DataType;
use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue as BytecodeTemplateValue;
use zinc_compiler::Bytecode;
use zinc_compiler::SourceString;

use self::entry::Entry;

///
/// The program resource.
///
pub struct Program {
    /// The original program source code, which can be returned back to its authors to be edited.
    pub source: SourceString,
    /// The compiled program entries representation, ready to be run on the Zinc VM.
    pub entries: HashMap<String, Entry>,
    /// The contract storage, if the program is a contract.
    pub contract_storage: Option<Vec<(String, DataType)>>,
}

impl Program {
    ///
    /// Creates a circuit without a contract storage.
    ///
    pub fn new_circuit(source: SourceString, entries: HashMap<String, Entry>) -> Self {
        Self {
            source,
            entries,
            contract_storage: None,
        }
    }

    ///
    /// Creates a contract with a contract storage.
    ///
    pub fn new_contract(
        source: SourceString,
        entries: HashMap<String, Entry>,
        contract_storage: Vec<(String, DataType)>,
    ) -> Self {
        Self {
            source,
            entries,
            contract_storage: Some(contract_storage),
        }
    }

    ///
    /// Returns a program entry.
    ///
    pub fn get_entry(&self, name: &str) -> Option<&Entry> {
        self.entries.get(name)
    }

    pub fn from_bytecode(bytecode: Bytecode) -> HashMap<String, Entry> {
        bytecode
            .into_entries()
            .into_iter()
            .map(|(name, entry)| {
                let program = BytecodeProgram::from_bytes(entry.into_bytecode().as_slice())
                    .expect(zinc_const::panic::DATA_SERIALIZATION);

                let input_type = program.input();
                let input_template = BytecodeTemplateValue::new(input_type.clone()).into_json();

                let output_type = program.output();
                let output_template = BytecodeTemplateValue::new(output_type.clone()).into_json();

                let entry = Entry::new(
                    program,
                    input_type,
                    input_template,
                    output_type,
                    output_template,
                );

                (name, entry)
            })
            .collect()
    }
}
