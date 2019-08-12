//!
//! The syntax parser.
//!

mod expression;
mod inputs;
mod r#type;
mod witness;

pub use self::expression::Parser as ExpressionParser;
pub use self::inputs::Parser as InputsParser;
pub use self::r#type::Parser as TypeParser;
pub use self::witness::Parser as WitnessParser;

use log::*;

use crate::lexical::TokenStream;
use crate::syntax::CircuitProgram;
use crate::Error;

pub fn parse(iterator: TokenStream) -> Result<CircuitProgram, Error> {
    let (iterator, inputs) = InputsParser::default().parse(iterator)?;
    let (iterator, witnesses) = WitnessParser::default().parse(iterator)?;

    let (_iterator, expression) = ExpressionParser::default().parse(iterator)?;
    let rpn = expression
        .into_iter()
        .map(|lexeme| lexeme.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    info!("{}", rpn);

    Ok(CircuitProgram { inputs, witnesses })
}
