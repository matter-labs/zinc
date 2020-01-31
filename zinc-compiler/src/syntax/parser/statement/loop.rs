//!
//! The loop statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::LoopStatement;
use crate::syntax::LoopStatementBuilder;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordFor,
    IndexIdentifier,
    KeywordIn,
    BoundsExpression,
    BlockExpressionOrKeywordWhile,
    WhileConditionExpression,
    BlockExpression,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordFor
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: LoopStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<LoopStatement, Error> {
        loop {
            match self.state {
                State::KeywordFor => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::For),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::IndexIdentifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["for"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::IndexIdentifier => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_index_identifier(identifier);
                            self.state = State::KeywordIn;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::KeywordIn => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::In),
                            ..
                        } => {
                            self.state = State::BoundsExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["in"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::BoundsExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_bounds_expression(expression);
                    self.state = State::BlockExpressionOrKeywordWhile;
                }
                State::BlockExpressionOrKeywordWhile => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        token
                        @
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            let block =
                                BlockExpressionParser::default().parse(stream, Some(token))?;
                            self.builder.set_block(block);
                            return Ok(self.builder.finish());
                        }
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::While),
                            ..
                        } => self.state = State::WhileConditionExpression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{", "while"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::WhileConditionExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_while_condition(expression);
                    self.state = State::BlockExpression;
                }
                State::BlockExpression => {
                    let expression =
                        BlockExpressionParser::default().parse(stream, self.next.take())?;
                    self.builder.set_block(expression);
                    return Ok(self.builder.finish());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::LoopStatement;

    #[test]
    fn ok_with_block() {
        let input = r#"for i in 0..=4 { 2 + 1 }"#;

        let expected = Ok(LoopStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 5), "i".to_owned()),
            Expression::new(
                Location::new(1, 10),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 10),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 10),
                                lexical::IntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 14),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 14),
                                lexical::IntegerLiteral::new_decimal("4".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 11),
                        ExpressionObject::Operator(ExpressionOperator::RangeInclusive),
                    ),
                ],
            ),
            None,
            BlockExpression::new(
                Location::new(1, 16),
                vec![],
                Some(Expression::new(
                    Location::new(1, 18),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 18),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 18),
                                    lexical::IntegerLiteral::new_decimal("2".to_owned()),
                                ),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 22),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 22),
                                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 20),
                            ExpressionObject::Operator(ExpressionOperator::Addition),
                        ),
                    ],
                )),
            ),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_with_empty_block() {
        let input = r#"for i in 0..4 {}"#;

        let expected = Ok(LoopStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 5), "i".to_owned()),
            Expression::new(
                Location::new(1, 10),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 10),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 10),
                                lexical::IntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 13),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 13),
                                lexical::IntegerLiteral::new_decimal("4".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 11),
                        ExpressionObject::Operator(ExpressionOperator::Range),
                    ),
                ],
            ),
            None,
            BlockExpression::new(Location::new(1, 15), vec![], None),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
