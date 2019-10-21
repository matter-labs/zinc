//!
//! The syntax parser.
//!

mod expression;
mod inputs;
mod statement;
mod r#type;
mod witnesses;

pub use self::expression::AccessOperandParser;
pub use self::expression::AddSubOperandParser;
pub use self::expression::AndOperandParser;
pub use self::expression::ArrayExpressionParser;
pub use self::expression::BlockExpressionParser;
pub use self::expression::CastingOperandParser;
pub use self::expression::ComparisonOperandParser;
pub use self::expression::ConditionalExpressionParser;
pub use self::expression::MulDivRemOperandParser;
pub use self::expression::OrOperandParser;
pub use self::expression::Parser as ExpressionParser;
pub use self::expression::PathExpressionParser;
pub use self::expression::StructureExpressionParser;
pub use self::expression::TupleExpressionParser;
pub use self::expression::MatchExpressionParser;
pub use self::expression::XorOperandParser;
pub use self::inputs::Parser as InputsParser;
pub use self::r#type::Parser as TypeParser;
pub use self::statement::DebugParser as DebugStatementParser;
pub use self::statement::LetParser as LetStatementParser;
pub use self::statement::LoopParser as LoopStatementParser;
pub use self::statement::Parser as StatementParser;
pub use self::statement::RequireParser as RequireStatementParser;
pub use self::witnesses::Parser as WitnessesParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::TokenStream;
use crate::syntax::CircuitProgram;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::Error;

pub struct Parser {}

impl Parser {
    pub fn parse(stream: TokenStream) -> Result<CircuitProgram, Error> {
        let stream = Rc::new(RefCell::new(stream));

        let inputs = InputsParser::default().parse(stream.clone())?;
        let witnesses = WitnessesParser::default().parse(stream.clone())?;

        let mut statements = Vec::new();
        loop {
            let peek = stream.borrow_mut().peek();
            match peek {
                Some(Ok(token)) => {
                    let (statement, is_unterminated) =
                        StatementParser::default().parse(stream.clone())?;
                    if let Statement::Expression(..) = statement {
                        if is_unterminated {
                            return Err(Error::Syntax(SyntaxError::UnterminatedExpressionAtRoot(
                                token.location,
                            )));
                        }
                    }

                    log::trace!("Statement: {:?}", statement);
                    statements.push(statement);
                }
                Some(Err(error)) => return Err(Error::Lexical(error)),
                None => break,
            }
        }

        Ok(CircuitProgram {
            inputs,
            witnesses,
            statements,
        })
    }
}
