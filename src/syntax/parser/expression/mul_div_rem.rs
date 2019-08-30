//!
//! The multiplication/division/remainder operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::CastingOperandParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    CastingFirstOperand,
    CastingOperator,
    CastingSecondOperand,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::CastingFirstOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    expression: Expression,
    operator: Option<(ExpressionOperator, Token)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::CastingFirstOperand => {
                    let rpn = CastingOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::CastingOperator;
                }
                State::CastingOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Keyword(Keyword::As),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((ExpressionOperator::Casting, token));
                            self.state = State::CastingSecondOperand;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::CastingSecondOperand => {
                    let token = match stream.borrow_mut().peek() {
                        Some(Ok(token)) => token,
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    };

                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.expression
                        .push_operand((ExpressionOperand::Type(r#type), token));
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::CastingOperator;
                }
                State::End => return Ok(self.expression),
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
    use crate::lexical::Keyword;
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let code = br#"42 as field "#;

        let expected = Expression::new(vec![
            ExpressionElement::new(
                ExpressionObject::Operand(ExpressionOperand::Literal(Literal::Integer(
                    IntegerLiteral::decimal(b"42".to_vec()),
                ))),
                Token::new(
                    Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"42".to_vec()))),
                    Location::new(1, 1),
                ),
            ),
            ExpressionElement::new(
                ExpressionObject::Operand(ExpressionOperand::Type(Type::new(
                    Location::new(1, 7),
                    TypeVariant::Field,
                ))),
                Token::new(Lexeme::Keyword(Keyword::Field), Location::new(1, 7)),
            ),
            ExpressionElement::new(
                ExpressionObject::Operator(ExpressionOperator::Casting),
                Token::new(Lexeme::Keyword(Keyword::As), Location::new(1, 4)),
            ),
        ]);

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
