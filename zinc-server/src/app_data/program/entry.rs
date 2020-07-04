//!
//! The program entry model.
//!

use serde_json::Value as JsonValue;

///
/// The program entry, which consists of the program representation, and the input and
/// output JSON template values.
///
#[derive(Debug, Clone)]
pub struct Entry {
    pub program: zinc_bytecode::Program,
    pub input_template: JsonValue,
    pub output_template: JsonValue,
}

impl Entry {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        program: zinc_bytecode::Program,
        input_template: JsonValue,
        output_template: JsonValue,
    ) -> Self {
        Self {
            program,
            input_template,
            output_template,
        }
    }

    ///
    /// Returns the input template.
    ///
    pub fn input_template(&self) -> JsonValue {
        self.input_template.to_owned()
    }

    ///
    /// Returns the output template.
    ///
    pub fn output_template(&self) -> JsonValue {
        self.output_template.to_owned()
    }
}
