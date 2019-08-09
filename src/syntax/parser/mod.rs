//!
//! The syntax parser.
//!

mod inputs;
mod r#type;
mod witness;

pub use self::inputs::InputsParser;
pub use self::r#type::TypeParser;
pub use self::witness::WitnessParser;

use crate::lexical::TokenStream;
use crate::syntax::CircuitProgram;
use crate::Error;

pub fn parse(iterator: TokenStream) -> Result<CircuitProgram, Error> {
    let (iterator, inputs) = InputsParser::default().parse(iterator)?;
    let (_iterator, witnesses) = WitnessParser::default().parse(iterator)?;

    Ok(CircuitProgram { inputs, witnesses })
}
