//!
//! The path expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::expression::structure::Parser as StructureExpressionParser;
use crate::syntax::parser::expression::terminal::Parser as TerminalOperandParser;
use crate::syntax::tree::expression::tree::builder::Builder as ExpressionTreeBuilder;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Terminal,
    DoubleColonOrStructureOrEnd,
}

impl Default for State {
    fn default() -> Self {
        Self::Terminal
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: ExpressionTreeBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), Error> {
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
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
