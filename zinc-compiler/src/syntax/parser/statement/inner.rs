//!
//! The inner statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ConstStatementParser;
use crate::syntax::ExpressionParser;
use crate::syntax::InnerStatement;
use crate::syntax::LetStatementParser;
use crate::syntax::LoopStatementParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Statement,
    SemicolonOptional,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Statement
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    statement: Option<InnerStatement>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(InnerStatement, Option<Token>, bool), Error> {
        loop {
            match self.state {
                State::Statement => {
                    self.statement = Some(
                        match match initial.take() {
                            Some(token) => token,
                            None => stream.borrow_mut().next()?,
                        } {
                            token @ Token {
                                lexeme: Lexeme::Keyword(Keyword::Let),
                                ..
                            } => {
                                let (statement, next) = LetStatementParser::default()
                                    .parse(stream.clone(), Some(token))?;
                                self.next = next;
                                self.state = State::End;
                                InnerStatement::Let(statement)
                            }
                            token @ Token {
                                lexeme: Lexeme::Keyword(Keyword::Const),
                                ..
                            } => {
                                let (statement, next) = ConstStatementParser::default()
                                    .parse(stream.clone(), Some(token))?;
                                self.next = next;
                                self.state = State::End;
                                InnerStatement::Const(statement)
                            }
                            token @ Token {
                                lexeme: Lexeme::Keyword(Keyword::For),
                                ..
                            } => {
                                let statement = LoopStatementParser::default()
                                    .parse(stream.clone(), Some(token))?;
                                self.state = State::End;
                                InnerStatement::Loop(statement)
                            }
                            token => {
                                let (expression, next) = ExpressionParser::default()
                                    .parse(stream.clone(), Some(token))?;
                                self.next = next;
                                self.state = State::SemicolonOptional;
                                InnerStatement::Expression(expression)
                            }
                        },
                    );
                }
                State::SemicolonOptional => {
                    match self
                        .next
                        .take()
                        .expect(crate::syntax::PANIC_VALUE_ALWAYS_EXISTS)
                    {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            return Ok((
                                self.statement
                                    .take()
                                    .expect(crate::syntax::PANIC_VALUE_ALWAYS_EXISTS),
                                None,
                                false,
                            ));
                        }
                        token => {
                            return Ok((
                                self.statement
                                    .take()
                                    .expect(crate::syntax::PANIC_VALUE_ALWAYS_EXISTS),
                                Some(token),
                                true,
                            ));
                        }
                    }
                }
                State::End => {
                    return Ok((
                        self.statement
                            .take()
                            .expect(crate::syntax::PANIC_VALUE_ALWAYS_EXISTS),
                        None,
                        false,
                    ))
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::InnerStatement;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::LetStatement;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_semicolon_terminated() {
        let input = r#"let mut a: u232 = 42;"#;

        let expected = Ok((
            InnerStatement::Let(LetStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 9), "a".to_owned()),
                true,
                Some(Type::new(
                    Location::new(1, 12),
                    TypeVariant::new_integer_unsigned(232),
                )),
                Expression::new(
                    Location::new(1, 19),
                    vec![ExpressionElement::new(
                        Location::new(1, 19),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 19),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    )],
                ),
            )),
            None,
            false,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_semicolon_unterminated() {
        let input = r#"{ 42 }"#;

        let expected = Ok((
            InnerStatement::Expression(Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Block(BlockExpression::new(
                        Location::new(1, 1),
                        vec![],
                        Some(Expression::new(
                            Location::new(1, 3),
                            vec![ExpressionElement::new(
                                Location::new(1, 3),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(1, 3),
                                        lexical::IntegerLiteral::new_decimal("42".to_owned()),
                                    ),
                                )),
                            )],
                        )),
                    ))),
                )],
            )),
            Some(Token::new(Lexeme::Eof, Location::new(1, 7))),
            true,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
