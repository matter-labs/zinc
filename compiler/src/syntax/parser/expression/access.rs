//!
//! The access operand parser.
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
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::StructureExpressionParser;
use crate::syntax::TupleExpressionParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Operand,
    BracketSquareLeftOrDotOrEnd,
    IndexExpression,
    BracketSquareRight,
    FieldDescriptor,
}

impl Default for State {
    fn default() -> Self {
        State::Operand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::Operand => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            let block = BlockExpressionParser::default().parse(stream.clone())?;
                            self.builder
                                .push_operand(block.location, ExpressionOperand::Block(block));
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            let conditional =
                                ConditionalExpressionParser::default().parse(stream.clone())?;
                            self.builder.push_operand(
                                conditional.location,
                                ExpressionOperand::Conditional(conditional),
                            );
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            let array = ArrayExpressionParser::default().parse(stream.clone())?;
                            self.builder
                                .push_operand(array.location, ExpressionOperand::Array(array));
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            let expression =
                                TupleExpressionParser::default().parse(stream.clone())?;
                            self.builder.extend_with_expression(expression);
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(literal),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.builder.set_location(location);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::Literal(Literal::new(location, literal)),
                            );
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(..),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            let expression =
                                StructureExpressionParser::default().parse(stream.clone())?;
                            self.builder.extend_with_expression(expression);
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["(", "{", "[", "if", "{literal}", "{identifier}"],
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
                State::BracketSquareLeftOrDotOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::Indexing));
                            self.state = State::IndexExpression;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Dot),
                            location,
                        })) => {
                            stream.borrow_mut().next();
                            self.operator = Some((location, ExpressionOperator::Field));
                            self.state = State::FieldDescriptor;
                        }
                        _ => return Ok(self.builder.finish()),
                    }
                }
                State::IndexExpression => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(literal @ lexical::Literal::Integer(..)),
                            location,
                        })) => {
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::Literal(Literal::new(location, literal)),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::BracketSquareRight;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}"],
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
                State::BracketSquareRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        })) => {
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["]"],
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
                State::FieldDescriptor => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(literal @ lexical::Literal::Integer(..)),
                            location,
                        })) => {
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::Literal(Literal::new(location, literal)),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::Identifier(Identifier::new(
                                    location,
                                    identifier.name,
                                )),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::BracketSquareLeftOrDotOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}", "{identifier}"],
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
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Literal;

    #[test]
    fn ok() {
        let input = r#"42 "#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 1),
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                    Location::new(1, 1),
                    lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
                ))),
            )],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
