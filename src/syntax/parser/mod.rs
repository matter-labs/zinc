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

pub fn parse(stream: TokenStream) -> Result<CircuitProgram, Error> {
    let (stream, inputs) = InputsParser::default().parse(stream)?;
    let (stream, witnesses) = WitnessParser::default().parse(stream)?;

    let mut statements = Vec::new();
    let mut stream = stream;
    loop {
        if stream.peek().is_none() {
            break;
        }
        let (s, statement) = StatementParser::default().parse(stream)?;
        stream = s;
        statements.push(statement);
    }

    Ok(CircuitProgram {
        inputs,
        witnesses,
        statements,
    })
}
