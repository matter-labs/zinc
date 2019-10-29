//!
//! The loop statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::LoopStatement;
use crate::syntax::LoopStatementBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordFor,
    IndexIdentifier,
    KeywordIn,
    RangeStart,
    RangeOperator,
    RangeEnd,
    BlockExpressionOrKeywordWhile,
    WhileCondition,
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
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
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
                    let next = stream.borrow_mut().next()?;
                    match next {
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
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::In),
                            ..
                        } => {
                            self.state = State::RangeStart;
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
                State::RangeStart => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Literal(Literal::Integer(integer)),
                            ..
                        } => {
                            self.builder.set_range_start(integer);
                            self.state = State::RangeOperator;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::RangeOperator => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleDot),
                            ..
                        } => {
                            self.state = State::RangeEnd;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleDotEquals),
                            ..
                        } => {
                            self.builder.set_range_inclusive();
                            self.state = State::RangeEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![".."],
                                lexeme,
                            )));
                        }
                    }
                }
                State::RangeEnd => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Literal(Literal::Integer(integer)),
                            ..
                        } => {
                            self.builder.set_range_end(integer);
                            self.state = State::BlockExpressionOrKeywordWhile;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::BlockExpressionOrKeywordWhile => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        token @ Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            let block = BlockExpressionParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.builder.set_block(block);
                            return Ok(self.builder.finish());
                        }
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::While),
                            ..
                        } => self.state = State::WhileCondition,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{", "while"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::WhileCondition => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_while_condition(expression);
                    self.state = State::BlockExpression;
                }
                State::BlockExpression => {
                    let expression =
                        BlockExpressionParser::default().parse(stream.clone(), self.next.take())?;
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;
    use crate::syntax::Literal;
    use crate::syntax::LoopStatement;
    use crate::Error;

    #[test]
    fn ok_with_block() {
        let input = r#"for i in 0..=4 { 2 + 1 };"#;

        let expected = Ok(LoopStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 5), "i".to_owned()),
            IntegerLiteral::new_decimal("0".to_owned()),
            IntegerLiteral::new_decimal("4".to_owned()),
            true,
            None,
            BlockExpression::new(
                Location::new(1, 16),
                vec![],
                Some(Expression::new(
                    Location::new(1, 18),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 18),
                            ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                                Location::new(1, 18),
                                lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                    "2".to_owned(),
                                )),
                            ))),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 22),
                            ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                                Location::new(1, 22),
                                lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                    "1".to_owned(),
                                )),
                            ))),
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
        let input = r#"for i in 0..4 {};"#;

        let expected = Ok(LoopStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 5), "i".to_owned()),
            IntegerLiteral::new_decimal("0".to_owned()),
            IntegerLiteral::new_decimal("4".to_owned()),
            false,
            None,
            BlockExpression::new(Location::new(1, 15), vec![], None),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn error_expected_integer_literal() {
        let input = r#"for i in 0..n {};"#;

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(1, 13),
            vec!["{integer}"],
            Lexeme::Identifier(lexical::Identifier::new("n".to_owned())),
        )));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
