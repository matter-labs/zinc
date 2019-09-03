//!
//! The casting operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Start,
    UnaryMulDivRemOperand,
    ParenthesisExpression,
    ParenthesisClose,
    BlockExpression,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    expression: OperatorExpression,
    operator: Option<(OperatorExpressionOperator, Token)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        loop {
            match self.state {
                State::Start => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((OperatorExpressionOperator::Not, token));
                            self.state = State::UnaryMulDivRemOperand;
                        }
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::Minus),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((OperatorExpressionOperator::Negation, token));
                            self.state = State::UnaryMulDivRemOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.state = State::ParenthesisExpression;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        })) => {
                            self.state = State::BlockExpression;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(literal),
                            ..
                        })) => {
                            let token = match stream.borrow_mut().next() {
                                Some(Ok(token)) => token,
                                Some(Err(error)) => return Err(Error::Lexical(error)),
                                None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                            };

                            self.expression
                                .push_operand((OperatorExpressionOperand::Literal(literal), token));
                            return Ok(self.expression);
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            let token = match stream.borrow_mut().next() {
                                Some(Ok(token)) => token,
                                Some(Err(error)) => return Err(Error::Lexical(error)),
                                None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                            };

                            let identifier = Identifier::new(location, identifier.name);
                            self.expression.push_operand((
                                OperatorExpressionOperand::Identifier(identifier),
                                token,
                            ));
                            return Ok(self.expression);
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["!", "-", "(", "{", "{literal}", "{identifier}"].to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::UnaryMulDivRemOperand => {
                    let rpn = Self::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    return Ok(self.expression);
                }
                State::ParenthesisExpression => {
                    match ExpressionParser::default().parse(stream.clone())? {
                        Expression::Operator(rpn) => self.expression.append(rpn),
                        Expression::Block(block) => {
                            let location = block.location;
                            self.expression.push_operand((
                                OperatorExpressionOperand::Block(block),
                                Token::new(Lexeme::Symbol(Symbol::BracketCurlyLeft), location),
                            ))
                        }
                    }
                    self.state = State::ParenthesisClose;
                }
                State::ParenthesisClose => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.expression);
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                [")"].to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::BlockExpression => {
                    let block = BlockExpressionParser::default().parse(stream.clone())?;
                    let location = block.location;
                    self.expression.push_operand((
                        OperatorExpressionOperand::Block(block),
                        Token::new(Lexeme::Symbol(Symbol::BracketCurlyLeft), location),
                    ));
                    return Ok(self.expression);
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;

    #[test]
    fn ok() {
        let code = br#"42 "#;

        let expected = OperatorExpression::new(vec![OperatorExpressionElement::new(
            OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                Literal::Integer(IntegerLiteral::decimal(b"42".to_vec())),
            )),
            Token::new(
                Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"42".to_vec()))),
                Location::new(1, 1),
            ),
        )]);

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
