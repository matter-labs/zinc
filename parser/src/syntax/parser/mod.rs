//!
//! The syntax parser.
//!

mod expression;
mod field;
mod field_list;
mod inputs;
mod outputs;
mod pattern;
mod statement;
mod r#type;
mod variant;
mod variant_list;
mod witnesses;

pub use self::expression::AccessOperandParser;
pub use self::expression::AddSubOperandParser;
pub use self::expression::AndOperandParser;
pub use self::expression::ArrayExpressionParser;
pub use self::expression::BlockExpressionParser;
pub use self::expression::CastingOperandParser;
pub use self::expression::ComparisonOperandParser;
pub use self::expression::ConditionalExpressionParser;
pub use self::expression::ListParser as ExpressionListParser;
pub use self::expression::MatchExpressionParser;
pub use self::expression::MulDivRemOperandParser;
pub use self::expression::OrOperandParser;
pub use self::expression::Parser as ExpressionParser;
pub use self::expression::PathExpressionParser;
pub use self::expression::StructureExpressionParser;
pub use self::expression::TupleExpressionParser;
pub use self::expression::XorOperandParser;
pub use self::field::Parser as FieldParser;
pub use self::field_list::Parser as FieldListParser;
pub use self::inputs::Parser as InputsParser;
pub use self::outputs::Parser as OutputsParser;
pub use self::pattern::Parser as PatternParser;
pub use self::r#type::Parser as TypeParser;
pub use self::statement::FnParser as FnStatementParser;
pub use self::statement::LetParser as LetStatementParser;
pub use self::statement::LoopParser as LoopStatementParser;
pub use self::statement::ModParser as ModStatementParser;
pub use self::statement::Parser as StatementParser;
pub use self::statement::UseParser as UseStatementParser;
pub use self::variant::Parser as VariantParser;
pub use self::variant_list::Parser as VariantListParser;
pub use self::witnesses::Parser as WitnessesParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::CircuitProgram;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::Error;

#[derive(Default)]
pub struct Parser {
    next: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, input: String) -> Result<CircuitProgram, Error> {
        let stream = TokenStream::new(input);
        let stream = Rc::new(RefCell::new(stream));

        let inputs = InputsParser::default().parse(stream.clone())?;
        let witnesses = WitnessesParser::default().parse(stream.clone())?;
        let outputs = OutputsParser::default().parse(stream.clone())?;

        let mut statements = Vec::new();
        loop {
            match match self.next.take() {
                Some(token) => token,
                None => stream.borrow_mut().next()?,
            } {
                Token {
                    lexeme: Lexeme::Eof,
                    ..
                } => break,
                token => {
                    let (statement, next, is_unterminated) =
                        StatementParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    if let Statement::Expression(ref expression) = statement {
                        if is_unterminated {
                            return Err(Error::Syntax(SyntaxError::ExpressionStatementAtRoot(
                                expression.location,
                            )));
                        }
                    }
                    log::trace!("Statement: {:?}", statement);
                    statements.push(statement);
                }
            }
        }

        Ok(CircuitProgram {
            inputs,
            witnesses,
            outputs,
            statements,
        })
    }
}
