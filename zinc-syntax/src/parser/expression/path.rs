//!
//! The path expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::expression::structure::Parser as StructureExpressionParser;
use crate::parser::expression::terminal::Parser as TerminalOperandParser;
use crate::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Terminal,
    /// The operand has been parsed and a `::` operator or structure literal is expected.
    DoubleColonOrStructureOrEnd,
}

impl Default for State {
    fn default() -> Self {
        Self::Terminal
    }
}

///
/// The path expression parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
    builder: ExpressionTreeBuilder,
}

impl Parser {
    ///
    /// Parses a path expression, which consists of several items.
    /// Can be terminated with a structure literal.
    ///
    /// 'value'
    /// 'path::to::Type'
    /// 'path::to::Structure { a: 42, b: 25 }'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Terminal => {
                    let (tree, next) =
                        TerminalOperandParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.eat(tree);
                    self.state = State::DoubleColonOrStructureOrEnd;
                }
                State::DoubleColonOrStructureOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            location,
                        } => {
                            self.builder
                                .eat_operator(ExpressionOperator::Path, location);
                            self.state = State::Terminal;
                        }
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            let look_ahead = stream.borrow_mut().look_ahead(2)?.to_owned();

                            return match look_ahead {
                                Token {
                                    lexeme: Lexeme::Symbol(Symbol::Colon),
                                    ..
                                } => {
                                    let location = token.location;

                                    self.builder
                                        .eat_operator(ExpressionOperator::Structure, location);

                                    let (expression, next) = StructureExpressionParser::default()
                                        .parse(stream.clone(), Some(token))?;
                                    self.builder.eat_operand(
                                        ExpressionOperand::Structure(expression),
                                        location,
                                    );

                                    Ok((self.builder.finish(), next))
                                }
                                _ => Ok((self.builder.finish(), Some(token))),
                            };
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
            }
        }
    }
}
