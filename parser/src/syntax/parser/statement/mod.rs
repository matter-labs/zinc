//!
//! The statement parser.
//!

mod debug;
mod r#enum;
mod r#let;
mod r#loop;
mod require;
mod r#struct;
mod r#type;

pub use self::debug::Parser as DebugParser;
pub use self::r#enum::Parser as EnumParser;
pub use self::r#let::Parser as LetParser;
pub use self::r#loop::Parser as LoopParser;
pub use self::r#struct::Parser as StructParser;
pub use self::r#type::Parser as TypeParser;
pub use self::require::Parser as RequireParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Statement;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Statement,
    Semicolon,
    SemicolonOptional,
}

impl Default for State {
    fn default() -> Self {
        State::Statement
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    statement: Option<Statement>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>, mut initial: Option<Token>) -> Result<(Statement, Option<Token>, bool), Error> {
        loop {
            match self.state {
                State::Statement => {
                    self.statement = Some(match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => return Ok((Statement::Empty, None, false)),
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Require),
                            ..
                        } => {
                            let statement = RequireParser::default()
                                .parse(stream.clone(), Some(token))?;;
                            self.state = State::Semicolon;
                            Statement::Require(statement)
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Let),
                            ..
                        } => {
                            let (statement, next) = LetParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.state = State::Semicolon;
                            Statement::Let(statement)
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::For),
                            ..
                        } => {
                            let statement = LoopParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.state = State::Semicolon;
                            Statement::Loop(statement)
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Type),
                            ..
                        } => {
                            let statement = TypeParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.state = State::Semicolon;
                            Statement::Type(statement)
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            ..
                        } => {
                            let (statement, next) = StructParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.state = State::Semicolon;
                            Statement::Struct(statement)
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Enum),
                            ..
                        } => {
                            let (statement, next) = EnumParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.state = State::Semicolon;
                            Statement::Enum(statement)
                        }
                        token @ Token {
                            lexeme: Lexeme::Keyword(Keyword::Debug),
                            ..
                        } => {
                            let statement = DebugParser::default()
                                .parse(stream.clone(), Some(token))?;
                            self.state = State::Semicolon;
                            Statement::Debug(statement)
                        }
                        token => {
                            let (expression, next) = ExpressionParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.state = State::SemicolonOptional;
                            Statement::Expression(expression)
                        }
                    });
                }
                State::Semicolon => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            return Ok((
                                self.statement.take().expect("Always contains a value"),
                                None,
                                false,
                            ))
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![";"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::SemicolonOptional => {
                    match self.next.take().expect("Always contains a value") {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            return Ok((
                                self.statement.take().expect("Always contains a value"),
                                None,
                                false,
                            ));
                        }
                        token => {
                            return Ok((
                                self.statement.take().expect("Always contains a value"),
                                Some(token),
                                true,
                            ));
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
    use crate::syntax::BlockExpression;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::LetStatement;
    use crate::syntax::Literal;
    use crate::syntax::Statement;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_empty() {
        let input = r#";"#;

        let expected = Ok((Statement::Empty, false));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_semicolon_terminated() {
        let input = r#"let mut a: u232 = 42;"#;

        let expected = Ok((
            Statement::Let(LetStatement::new(
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
                        ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                            Location::new(1, 19),
                            lexical::Literal::Integer(IntegerLiteral::new_decimal("42".to_owned())),
                        ))),
                    )],
                ),
            )),
            false,
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_semicolon_unterminated() {
        let input = r#"{ 42 } "#;

        let expected = Ok((
            Statement::Expression(Expression::new(
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
                                ExpressionObject::Operand(ExpressionOperand::Literal(
                                    Literal::new(
                                        Location::new(1, 3),
                                        lexical::Literal::Integer(IntegerLiteral::new_decimal(
                                            "42".to_owned(),
                                        )),
                                    ),
                                )),
                            )],
                        )),
                    ))),
                )],
            )),
            true,
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
