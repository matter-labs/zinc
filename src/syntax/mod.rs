//!
//! The syntax analysis.
//!

mod circuit;
mod error;
mod input;
mod parser;
mod r#type;
mod witness;

pub use self::circuit::CircuitProgram;
pub use self::error::Error;
pub use self::input::Builder as InputBuilder;
pub use self::input::Input;
pub use self::parser::parse;
pub use self::parser::TypeParser;
pub use self::r#type::Builder as TypeBuilder;
pub use self::r#type::Type;
pub use self::witness::Builder as WitnessBuilder;
pub use self::witness::Witness;
