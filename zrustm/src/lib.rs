#[macro_use]
extern crate zrust_bytecode;

mod vm;
mod element;
mod instructions;

mod facade;
pub use facade::*;
pub use vm::RuntimeError;
