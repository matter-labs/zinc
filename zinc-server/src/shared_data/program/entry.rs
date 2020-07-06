//!
//! The program entry model.
//!

use serde_json::Value as JsonValue;

use zinc_bytecode::DataType;
use zinc_bytecode::Program;

///
/// The program entry, which consists of the program representation, and the input and
/// output type structures and JSON template values.
///
#[derive(Debug, Clone)]
pub struct Entry {
    pub program: Program,
    pub input_type: DataType,
    pub input_template: JsonValue,
    pub output_type: DataType,
    pub output_template: JsonValue,
}

impl Entry {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        program: Program,
        input_type: DataType,
        input_template: JsonValue,
        output_type: DataType,
        output_template: JsonValue,
    ) -> Self {
        Self {
            program,
            input_type,
            input_template,
            output_type,
            output_template,
        }
    }
}
