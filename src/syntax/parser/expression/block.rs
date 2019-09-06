//!
//! The block expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpression;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::syntax::StatementParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketOpen,
    StatementOrBracketClose,
    BracketClose,
}

impl Default for State {
    fn default() -> Self {
        State::BracketOpen
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    block: BlockExpression,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<BlockExpression, Error> {
        loop {
            match self.state {
                State::BracketOpen => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                        location,
                    })) => {
                        self.block.location = location;
                        self.state = State::StatementOrBracketClose;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["{"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::StatementOrBracketClose => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.block);
                        }
                        Some(Ok(..)) => {
                            let (statement, is_unterminated) =
                                StatementParser::default().parse(stream.clone())?;
                            match statement {
                                Statement::Expression(expression) => {
                                    if is_unterminated {
                                        self.block.expression = Some(Box::new(expression));
                                        self.state = State::BracketClose;
                                    } else {
                                        self.block
                                            .statements
                                            .push(Statement::Expression(expression));
                                    }
                                }
                                statement => self.block.statements.push(statement),
                            }
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::BracketClose => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                        ..
                    })) => return Ok(self.block),
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["}"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
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
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::DebugStatement;
    use crate::syntax::Expression;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;
    use crate::syntax::OperatorExpressionOperator;
    use crate::syntax::Statement;

    #[test]
    fn ok() {
        let code = r#"{ debug(42); 2 + 1 }"#;

        let expected = BlockExpression::new(
            Location::new(1, 1),
            vec![Statement::Debug(DebugStatement::new(
                Location::new(1, 3),
                Expression::Operator(OperatorExpression::new(vec![
                    OperatorExpressionElement::new(
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                        )),
                        Token::new(
                            Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(
                                "42".to_owned(),
                            ))),
                            Location::new(1, 9),
                        ),
                    ),
                ])),
            ))],
            Some(Expression::Operator(OperatorExpression::new(vec![
                OperatorExpressionElement::new(
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::Integer(IntegerLiteral::decimal("2".to_owned())),
                    )),
                    Token::new(
                        Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal("2".to_owned()))),
                        Location::new(1, 14),
                    ),
                ),
                OperatorExpressionElement::new(
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::Integer(IntegerLiteral::decimal("1".to_owned())),
                    )),
                    Token::new(
                        Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal("1".to_owned()))),
                        Location::new(1, 18),
                    ),
                ),
                OperatorExpressionElement::new(
                    OperatorExpressionObject::Operator(OperatorExpressionOperator::Addition),
                    Token::new(Lexeme::Symbol(Symbol::Plus), Location::new(1, 16)),
                ),
            ]))),
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
