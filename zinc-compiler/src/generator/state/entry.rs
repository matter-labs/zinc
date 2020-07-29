//!
//! The Zinc VM bytecode compiled entry.
//!

///
/// A compiled application entry, which consists of the bytecode bytes, witness template bytes,
/// and public data template bytes.
///
/// Unit tests do not contain witness and public data.
///
#[derive(Debug)]
pub enum Entry {
    /// The default circuit or contract entry.
    Default {
        /// The bytecode file with metadata as a byte vector.
        bytecode: Vec<u8>,
        /// The witness JSON template as a byte vector.
        witness_template: Vec<u8>,
        /// The public data JSON template as a byte vector.
        public_data_template: Vec<u8>,
    },
    /// The unit test entry.
    Test {
        /// The bytecode file with metadata as a byte vector.
        bytecode: Vec<u8>,
    },
}

impl Entry {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        bytecode: Vec<u8>,
        witness_template: Vec<u8>,
        public_data_template: Vec<u8>,
    ) -> Self {
        Self::Default {
            bytecode,
            witness_template,
            public_data_template,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_test(bytecode: Vec<u8>) -> Self {
        Self::Test { bytecode }
    }

    ///
    /// Converts the entry into its bytecode bytes.
    ///
    pub fn into_bytecode(self) -> Vec<u8> {
        match self {
            Self::Default { bytecode, .. } => bytecode,
            Self::Test { bytecode } => bytecode,
        }
    }
}
