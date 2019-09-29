//!
//! The indexing operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ArrayExpressionParser;
use crate::syntax::BlockExpressionParser;
use crate::syntax::ConditionalExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionParser;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::OperatorExpression;
use crate::syntax::OperatorExpressionBuilder;
use crate::syntax::OperatorExpressionOperand;
use crate::syntax::OperatorExpressionOperator;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Operand,
    ParenthesisExpressionOrParenthesisRight,
    ParenthesisRight,
    BlockExpression,
    ConditionalExpression,
    ArrayExpression,
    BracketSquareOpenOrEnd,
    IndexExpression,
    BracketSquareClose,
}

impl Default for State {
    fn default() -> Self {
        State::Operand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: OperatorExpressionBuilder,
    operator: Option<(Location, OperatorExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<OperatorExpression, Error> {
        loop {
            match self.state {
                State::Operand => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.builder.set_location(location);
                            self.state = State::ParenthesisExpressionOrParenthesisRight;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::BlockExpression;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::ConditionalExpression;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::ArrayExpression;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(literal),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.builder.set_location(location);
                            self.builder.push_operand(
                                location,
                                OperatorExpressionOperand::Literal(Literal::new(location, literal)),
                            );
                            self.state = State::BracketSquareOpenOrEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.builder.set_location(location);
                            self.builder.push_operand(
                                location,
                                OperatorExpressionOperand::Identifier(Identifier::new(
                                    location,
                                    identifier.name,
                                )),
                            );
                            self.state = State::BracketSquareOpenOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["(", "{", "[", "if", "{literal}", "{identifier}"].to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::ParenthesisExpressionOrParenthesisRight => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.builder.push_operand(
                                location,
                                OperatorExpressionOperand::Literal(Literal::new(
                                    location,
                                    lexical::Literal::Void,
                                )),
                            );
                            self.state = State::BracketSquareOpenOrEnd;
                        }
                        Some(Ok(Token { location, .. })) => {
                            match ExpressionParser::default().parse(stream.clone())? {
                                Expression::Operator(rpn) => self.builder.append_expression(rpn),
                                Expression::Block(block) => self.builder.push_operand(
                                    location,
                                    OperatorExpressionOperand::Block(block),
                                ),
                                Expression::Conditional(conditional) => self.builder.push_operand(
                                    location,
                                    OperatorExpressionOperand::Conditional(conditional),
                                ),
                            }
                            self.state = State::ParenthesisRight;
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::ParenthesisRight => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.state = State::BracketSquareOpenOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                [")"].to_vec(),
                                lexeme,
                            )))
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
                    self.builder
                        .push_operand(block.location, OperatorExpressionOperand::Block(block));
                    self.state = State::BracketSquareOpenOrEnd;
                }
                State::ConditionalExpression => {
                    let conditional =
                        ConditionalExpressionParser::default().parse(stream.clone())?;
                    self.builder.push_operand(
                        conditional.location,
                        OperatorExpressionOperand::Conditional(conditional),
                    );
                    self.state = State::BracketSquareOpenOrEnd;
                }
                State::ArrayExpression => {
                    let array = ArrayExpressionParser::default().parse(stream.clone())?;
                    self.builder
                        .push_operand(array.location, OperatorExpressionOperand::Array(array));
                    self.state = State::BracketSquareOpenOrEnd;
                }
                State::BracketSquareOpenOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, OperatorExpressionOperator::Indexing));
                            self.state = State::IndexExpression;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
                }
                State::IndexExpression => {
                    match ExpressionParser::default().parse(stream.clone())? {
                        Expression::Operator(rpn) => self.builder.append_expression(rpn),
                        Expression::Block(block) => self
                            .builder
                            .push_operand(block.location, OperatorExpressionOperand::Block(block)),
                        Expression::Conditional(conditional) => self.builder.push_operand(
                            conditional.location,
                            OperatorExpressionOperand::Conditional(conditional),
                        ),
                    }
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::BracketSquareClose;
                }
                State::BracketSquareClose => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => {
                            self.state = State::BracketSquareOpenOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["]"].to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
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
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;

    #[test]
    fn ok() {
        let code = r#"42 "#;

        let expected = OperatorExpression::new(
            Location::new(1, 1),
            vec![OperatorExpressionElement::new(
                Location::new(1, 1),
                OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                    Literal::new(
                        Location::new(1, 1),
                        lexical::Literal::Integer(IntegerLiteral::decimal("42".to_owned())),
                    ),
                )),
            )],
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
