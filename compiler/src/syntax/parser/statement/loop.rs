//!
//! The debug statement parser.
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
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<LoopStatement, Error> {
        loop {
            match self.state {
                State::KeywordFor => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::For),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::IndexIdentifier;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["for"].to_vec(),
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::IndexIdentifier => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_index_identifier(identifier);
                            self.state = State::KeywordIn;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["{identifier}"].to_vec(),
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::KeywordIn => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::In),
                            ..
                        })) => {
                            self.state = State::RangeStart;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["in"].to_vec(),
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::RangeStart => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(Literal::Integer(integer)),
                            ..
                        })) => {
                            self.builder.set_range_start(integer);
                            self.state = State::RangeOperator;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["{integer}"].to_vec(),
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::RangeOperator => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleDot),
                            ..
                        })) => {
                            self.state = State::RangeEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleDotEquals),
                            ..
                        })) => {
                            self.builder.set_range_inclusive();
                            self.state = State::RangeEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                [".."].to_vec(),
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::RangeEnd => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(Literal::Integer(integer)),
                            ..
                        })) => {
                            self.builder.set_range_end(integer);
                            self.state = State::BlockExpression;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["{integer}"].to_vec(),
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::BlockExpression => {
                    let block = BlockExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_block(block);
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
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::DebugStatement;
    use crate::syntax::Expression;
    use crate::syntax::Identifier;
    use crate::syntax::Literal;
    use crate::syntax::LoopStatement;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;
    use crate::syntax::Statement;

    #[test]
    fn ok() {
        let code = r#"for i in 0..=4 { debug(42); 2 + 1 };"#;

        let expected = LoopStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 5), "i".to_owned()),
            0,
            4,
            true,
            BlockExpression::new(
                Location::new(1, 16),
                vec![Statement::Debug(DebugStatement::new(
                    Location::new(1, 18),
                    Expression::Operator(OperatorExpression::new(
                        Location::new(1, 24),
                        vec![OperatorExpressionElement::new(
                            Location::new(1, 24),
                            OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                                Literal::new(
                                    Location::new(1, 24),
                                    lexical::Literal::Integer(IntegerLiteral::decimal(
                                        "42".to_owned(),
                                    )),
                                ),
                            )),
                        )],
                    )),
                ))],
                Some(Expression::Operator(OperatorExpression::new(
                    Location::new(1, 29),
                    vec![
                        OperatorExpressionElement::new(
                            Location::new(1, 29),
                            OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                                Literal::new(
                                    Location::new(1, 29),
                                    lexical::Literal::Integer(IntegerLiteral::decimal(
                                        "2".to_owned(),
                                    )),
                                ),
                            )),
                        ),
                        OperatorExpressionElement::new(
                            Location::new(1, 33),
                            OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                                Literal::new(
                                    Location::new(1, 33),
                                    lexical::Literal::Integer(IntegerLiteral::decimal(
                                        "1".to_owned(),
                                    )),
                                ),
                            )),
                        ),
                        OperatorExpressionElement::new(
                            Location::new(1, 31),
                            OperatorExpressionObject::Operator(
                                OperatorExpressionOperator::Addition,
                            ),
                        ),
                    ],
                ))),
            ),
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
