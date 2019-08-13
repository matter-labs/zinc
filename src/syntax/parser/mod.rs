//!
//! The syntax parser.
//!

mod expression;
mod inputs;
mod statement;
mod r#type;
mod witness;

pub use self::expression::Parser as ExpressionParser;
pub use self::inputs::Parser as InputsParser;
pub use self::r#type::Parser as TypeParser;
pub use self::statement::Parser as StatementParser;
pub use self::witness::Parser as WitnessParser;

use crate::lexical::TokenStream;
use crate::syntax::CircuitProgram;
use crate::Error;

pub fn parse(iterator: TokenStream) -> Result<CircuitProgram, Error> {
    let (iterator, inputs) = InputsParser::default().parse(iterator)?;
    let (iterator, witnesses) = WitnessParser::default().parse(iterator)?;

    let mut statements = Vec::new();
    let (_iterator, statement) = StatementParser::default().parse(iterator)?;
    statements.push(statement);

    Ok(CircuitProgram {
        inputs,
        witnesses,
        statements,
    })
}
