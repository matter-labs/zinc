//!
//! ZRust bytecode library.
//!

mod instruction;

pub use self::instruction::Error as InstructionError;
pub use self::instruction::Instruction;
pub use self::instruction::OperationCode;
pub use self::instruction::Push;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;

pub const BITLENGTH_BYTE: usize = 8;

#[derive(Debug, Fail)]
pub enum InputError {
    #[fail(display = "Opening: {}", _0)]
    Opening(std::io::Error),
    #[fail(display = "Metadata: {}", _0)]
    Metadata(std::io::Error),
    #[fail(display = "Reading: {}", _0)]
    Reading(std::io::Error),
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Instruction: {}", _0)]
    Instruction(InstructionError),
}

pub fn from_file(path: PathBuf) -> Result<Vec<Instruction>, Error> {
    let mut input_file = File::open(&path)
        .map_err(InputError::Opening)
        .map_err(Error::Input)?;
    let size = input_file
        .metadata()
        .map_err(InputError::Metadata)
        .map_err(Error::Input)?
        .len() as usize;
    let mut input = Vec::with_capacity(size);
    input_file
        .read_to_end(&mut input)
        .map_err(InputError::Reading)
        .map_err(Error::Input)?;

    let mut cursor = 0;
    let mut instructions = Vec::new();
    loop {
        if cursor == input.len() {
            break;
        }

        let (instruction, offset) =
            Instruction::new_from_slice(&input[cursor..]).map_err(Error::Instruction)?;
        cursor += offset;
        instructions.push(instruction);
    }

    Ok(instructions)
}
