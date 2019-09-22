//!
//! The conditional expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpressionParser;
use crate::syntax::ConditionalExpression;
use crate::syntax::ConditionalExpressionBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordIf,
    Condition,
    MainBlock,
    ElseKeywordOrEnd,
    KeywordIfOrElseBlock,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordIf
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ConditionalExpressionBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
    ) -> Result<ConditionalExpression, Error> {
        loop {
            match self.state {
                State::KeywordIf => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(Keyword::If),
                        location,
                    })) => {
                        self.builder.set_location(location);
                        self.state = State::Condition;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["if"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Condition => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_condition(expression);
                    self.state = State::MainBlock;
                }
                State::MainBlock => {
                    let block = BlockExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_main_block(block);
                    self.state = State::ElseKeywordOrEnd;
                }
                State::ElseKeywordOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Else),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.state = State::KeywordIfOrElseBlock;
                        }
                        Some(Ok(..)) => return Ok(self.builder.finish()),
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::KeywordIfOrElseBlock => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::If),
                            ..
                        })) => {
                            let conditional = Self::default().parse(stream.clone())?;
                            self.builder.set_else_if(conditional);
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        })) => {
                            let block = BlockExpressionParser::default().parse(stream.clone())?;
                            self.builder.set_else_block(block);
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["if", "{"].to_vec(),
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
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
    use crate::lexical::BooleanLiteral;
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::ConditionalExpression;
    use crate::syntax::Expression;
    use crate::syntax::Literal;
    use crate::syntax::OperatorExpression;
    use crate::syntax::OperatorExpressionElement;
    use crate::syntax::OperatorExpressionObject;
    use crate::syntax::OperatorExpressionOperand;

    #[test]
    fn ok() {
        let code = r#"if true { 1 } else if false { 2 } else { 3 }"#;

        let expected = ConditionalExpression::new(
            Location::new(1, 1),
            Expression::Operator(OperatorExpression::new(
                Location::new(1, 4),
                vec![OperatorExpressionElement::new(
                    Location::new(1, 4),
                    OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                        Literal::new(
                            Location::new(1, 4),
                            lexical::Literal::Boolean(BooleanLiteral::True),
                        ),
                    )),
                )],
            )),
            BlockExpression::new(
                Location::new(1, 9),
                vec![],
                Some(Expression::Operator(OperatorExpression::new(
                    Location::new(1, 11),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 11),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 11),
                                lexical::Literal::Integer(IntegerLiteral::decimal("1".to_owned())),
                            ),
                        )),
                    )],
                ))),
            ),
            Some(ConditionalExpression::new(
                Location::new(1, 20),
                Expression::Operator(OperatorExpression::new(
                    Location::new(1, 23),
                    vec![OperatorExpressionElement::new(
                        Location::new(1, 23),
                        OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                            Literal::new(
                                Location::new(1, 23),
                                lexical::Literal::Boolean(BooleanLiteral::False),
                            ),
                        )),
                    )],
                )),
                BlockExpression::new(
                    Location::new(1, 29),
                    vec![],
                    Some(Expression::Operator(OperatorExpression::new(
                        Location::new(1, 31),
                        vec![OperatorExpressionElement::new(
                            Location::new(1, 31),
                            OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                                Literal::new(
                                    Location::new(1, 31),
                                    lexical::Literal::Integer(IntegerLiteral::decimal(
                                        "2".to_owned(),
                                    )),
                                ),
                            )),
                        )],
                    ))),
                ),
                None,
                Some(BlockExpression::new(
                    Location::new(1, 40),
                    vec![],
                    Some(Expression::Operator(OperatorExpression::new(
                        Location::new(1, 42),
                        vec![OperatorExpressionElement::new(
                            Location::new(1, 42),
                            OperatorExpressionObject::Operand(OperatorExpressionOperand::Literal(
                                Literal::new(
                                    Location::new(1, 42),
                                    lexical::Literal::Integer(IntegerLiteral::decimal(
                                        "3".to_owned(),
                                    )),
                                ),
                            )),
                        )],
                    ))),
                )),
            )),
            None,
        );

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_owned()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
