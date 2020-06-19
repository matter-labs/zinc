//!
//! The Zinc VM bytecode compiled entry.
//!

///
/// A compiled application entry, which consists of the bytecode bytes, witness template bytes,
/// and public data template bytes.
///
/// Unit tests do not contrain witness and public data.
///
#[derive(Debug)]
pub enum Entry {
    Default {
        bytecode: Vec<u8>,
        witness_template: Vec<u8>,
        public_data_template: Vec<u8>,
    },
    Test {
        bytecode: Vec<u8>,
    },
}

impl Entry {
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

    pub fn new_test(bytecode: Vec<u8>) -> Self {
        Self::Test { bytecode }
    }

    pub fn into_bytecode(self) -> Vec<u8> {
        match self {
            Self::Default { bytecode, .. } => bytecode,
            Self::Test { bytecode } => bytecode,
        }
    }
}
