//!
//! The type statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::r#type::Parser as TypeParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#type::builder::Builder as TypeStatementBuilder;
use crate::syntax::tree::statement::r#type::Statement as TypeStatement;

static HINT_EXPECTED_IDENTIFIER: &str =
    "type alias must have an identifier, e.g. `type Complex = (u8, field);`";
static HINT_EXPECTED_TYPE: &str =
    "type alias must be initialized, e.g. `type Complex = (u8, field);`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordType,
    Identifier,
    Equals,
    Type,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordType
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: TypeStatementBuilder,
}

impl Parser {
    ///
    /// Parses a 'type' statement.
    ///
    /// 'type ArrayIndex = u64;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(TypeStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordType => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Type),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["type"],
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
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::Equals;
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
                State::Equals => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
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
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(Error::Syntax(
                            SyntaxError::expected_one_of(location, vec![";"], lexeme, None),
                        )),
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
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::r#type::Statement as TypeStatement;

    #[test]
    fn ok() {
        let input = r#"type X = field;"#;

        let expected = Ok((
            TypeStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 6), "X".to_owned()),
                Type::new(Location::new(1, 10), TypeVariant::Field),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"type = field;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 6),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"type Data;"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 10),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"type Data = field"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 18),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
