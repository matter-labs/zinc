//!
//! The syntax parser.
//!

mod expression;
mod inputs;
mod statement;
mod r#type;
mod witnesses;

pub use self::expression::AddSubOperatorOperandParser;
pub use self::expression::AndOperatorOperandParser;
pub use self::expression::CastingOperatorOperandParser;
pub use self::expression::ComparisonOperatorOperandParser;
pub use self::expression::MulDivRemOperatorOperandParser;
pub use self::expression::OperatorExpressionParser;
pub use self::expression::OrOperatorOperandParser;
pub use self::expression::Parser as ExpressionParser;
pub use self::expression::XorOperatorOperandParser;
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
