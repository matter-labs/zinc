//!
//! The syntax parser.
//!

mod expression;
mod inputs;
mod statement;
mod r#type;
mod witnesses;

pub use self::expression::AddSubOperandParser;
pub use self::expression::AndOperandParser;
pub use self::expression::CastingOperandParser;
pub use self::expression::ComparisonOperandParser;
pub use self::expression::MulDivRemOperandParser;
pub use self::expression::OrOperandParser;
pub use self::expression::Parser as ExpressionParser;
pub use self::expression::XorOperandParser;
pub use self::inputs::Parser as InputsParser;
pub use self::r#type::Parser as TypeParser;
pub use self::statement::Parser as StatementParser;
pub use self::witnesses::Parser as WitnessParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::TokenStream;
use crate::syntax::CircuitProgram;
use crate::Error;

pub fn parse(stream: TokenStream) -> Result<CircuitProgram, Error> {
    let stream = Rc::new(RefCell::new(stream));

    let inputs = InputsParser::default().parse(stream.clone())?;
    let witnesses = WitnessParser::default().parse(stream.clone())?;

    let mut statements = Vec::new();
    loop {
        if stream.borrow_mut().peek().is_none() {
            break;
        }
        let statement = StatementParser::default().parse(stream.clone())?;
        log::trace!("Statement: {:?}", statement);
        statements.push(statement);
    }

    Ok(CircuitProgram {
        inputs,
        witnesses,
        statements,
    })
}
