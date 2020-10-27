//!
//! The Zinc build representation.
//!

pub mod input;

use self::input::Input;

///
/// A compiled application data, which consists of the bytecode, input and
/// output template files.
///
#[derive(Debug)]
pub struct Build {
    /// The bytecode file with metadata as a byte array.
    pub bytecode: Vec<u8>,
    /// The input file data.
    pub input: Input,
}

impl Build {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(bytecode: Vec<u8>, input: Input) -> Self {
        Self { bytecode, input }
    }

    ///
    /// Extracts the bytecode file bytes.
    ///
    pub fn into_bytecode(self) -> Vec<u8> {
        self.bytecode
    }
}
