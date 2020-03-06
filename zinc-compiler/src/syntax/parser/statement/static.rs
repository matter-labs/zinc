//!
//! The static statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::parser::r#type::Parser as TypeParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#static::builder::Builder as StaticStatementBuilder;
use crate::syntax::tree::statement::r#static::Statement as StaticStatement;

static HINT_EXPECTED_IDENTIFIER: &str =
    "static data must have an identifier, e.g. `static DATA: u8 = 42;`";
static HINT_EXPECTED_TYPE: &str = "static data must have a type, e.g. `static DATA: u8 = 42;`";
static HINT_EXPECTED_VALUE: &str = "static data must be initialized, e.g. `static DATA: u8 = 42;`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordStatic,
    Identifier,
    Colon,
    Type,
    Equals,
    Expression,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordStatic
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: StaticStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(StaticStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordStatic => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Static),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["static"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_identifier(identifier);
                            self.state = State::Colon;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::Colon => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => self.state = State::Type,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_type(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_TYPE),
                            )));
                        }
                    }
                }
                State::Type => {
                    let (r#type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_type(r#type);
                    self.state = State::Equals;
                }
                State::Equals => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            ..
                        } => self.state = State::Expression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_value(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_VALUE),
                            )));
                        }
                    }
                }
                State::Expression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), None)?;
                    self.builder.set_expression(expression);
                    self.next = next;
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => {
                            Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec![";"],
                                lexeme,
                                None,
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
    use crate::error::Error;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::r#static::Statement as StaticStatement;

    #[test]
    fn ok() {
        let input = r#"static A: u64 = 42;"#;

        let expected = Ok((
            StaticStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 8), "A".to_owned()),
                Type::new(Location::new(1, 11), TypeVariant::integer_unsigned(64)),
                Expression::new(
                    Location::new(1, 17),
                    vec![ExpressionElement::new(
                        Location::new(1, 17),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 17),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    )],
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"static = 42;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 8),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"static VALUE = 42;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 14),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value() {
        let input = r#"static A: u64;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_value(
            Location::new(1, 14),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = "static A: u64 = 42";

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 19),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
