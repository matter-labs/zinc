mod stack;
mod vm;
mod vm_instruction;

pub mod instructions;

pub use stack::{Stack, Primitive};
pub use vm::{VirtualMachine, RuntimeError};
pub use vm_instruction::{VMInstruction, decode_all_vm_instructions};
