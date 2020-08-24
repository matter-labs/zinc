//!
//! The Zinc build bytes representation.
//!

use std::collections::HashMap;

///
/// A compiled application data, which consists of the bytecode, witness template,
/// and public data template files.
///
#[derive(Debug)]
pub enum Bytes {
    /// The circuit byte representation.
    Circuit {
        /// The bytecode file with metadata as a byte array.
        bytecode: Vec<u8>,
        /// The witness JSON template file as a byte array.
        input_template: Vec<u8>,
        /// The public data JSON template file as a byte array.
        output_template: Vec<u8>,
    },
    /// The contract byte representation.
    Contract {
        /// The bytecode file with metadata as a byte array.
        bytecode: Vec<u8>,
        /// The witness JSON template files as byte arrays.
        input_templates: HashMap<String, Vec<u8>>,
        /// The public data JSON template files as byte arrays.
        output_templates: HashMap<String, Vec<u8>>,
    },
}

impl Bytes {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_circuit(
        bytecode: Vec<u8>,
        input_template: Vec<u8>,
        output_template: Vec<u8>,
    ) -> Self {
        Self::Circuit {
            bytecode,
            input_template,
            output_template,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_contract(
        bytecode: Vec<u8>,
        input_templates: HashMap<String, Vec<u8>>,
        output_templates: HashMap<String, Vec<u8>>,
    ) -> Self {
        Self::Contract {
            bytecode,
            input_templates,
            output_templates,
        }
    }

    ///
    /// Extracts the bytecode file bytes.
    ///
    pub fn into_bytecode(self) -> Vec<u8> {
        match self {
            Self::Circuit { bytecode, .. } => bytecode,
            Self::Contract { bytecode, .. } => bytecode,
        }
    }
}
