//!
//! The Zinc VM bytecode compiled entry.
//!

///
/// The compiled application entry, which consists of the bytecode bytes, witness template bytes,
/// and public data template bytes.
///
pub struct CompiledEntry {
    pub bytecode: Vec<u8>,
    pub witness_template: Vec<u8>,
    pub public_data_template: Vec<u8>,
}

impl CompiledEntry {
    pub fn new(
        bytecode: Vec<u8>,
        witness_template: Vec<u8>,
        public_data_template: Vec<u8>,
    ) -> Self {
        Self {
            bytecode,
            witness_template,
            public_data_template,
        }
    }
}
