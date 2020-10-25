//!
//! The `type` statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#type::builder::Builder as TypeStatementBuilder;
use crate::tree::statement::r#type::Statement as TypeStatement;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str =
    "type alias must have an identifier, e.g. `type Complex = (u8, field);`";
/// The missing type error hint.
pub static HINT_EXPECTED_TYPE: &str =
    "type alias must be initialized, e.g. `type Complex = (u8, field);`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordType,
    /// The `type` has been parsed so far.
    Identifier,
    /// The `type {identifier}` has been parsed so far.
    Equals,
    /// The `type {identifier} =` has been parsed so far.
    Type,
    /// The `type {identifier} = {type}` has been parsed so far.
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordType
    }
}

///
/// The `type` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The token returned from a subparser.
    next: Option<Token>,
    /// The builder of the parsed value.
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
        initial: Option<Token>,
    ) -> Result<(TypeStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordType => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Type),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["type"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::Equals;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_IDENTIFIER),
                            )));
                        }
                    }
                }
                State::Equals => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Equals),
                            ..
                        } => self.state = State::Type,
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_type(
                                location,
                                lexeme,
                                Some(HINT_EXPECTED_TYPE),
                            )));
                        }
                    }
                }
                State::Type => {
                    let (r#type, next) =
                        TypeParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_type(r#type);
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    return match crate::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
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
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::identifier::Identifier;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;
    use crate::tree::statement::r#type::Statement as TypeStatement;

    #[test]
    fn ok() {
        let input = r#"type X = field;"#;

        let expected = Ok((
            TypeStatement::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 6), "X".to_owned()),
                Type::new(Location::test(1, 10), TypeVariant::field()),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"type = field;"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 6),
            Lexeme::Symbol(Symbol::Equals),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"type Data;"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_type(
            Location::test(1, 10),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"type Data = field"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 18),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
