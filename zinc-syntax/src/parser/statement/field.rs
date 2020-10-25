//!
//! The `field` statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::field::builder::Builder as FieldStatementBuilder;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str = "field must have an identifier, e.g. `data: u8;`";
/// The missing type error hint.
pub static HINT_EXPECTED_TYPE: &str = "field must have a type, e.g. `data: u8;`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    Identifier,
    /// The `{identifier}` has been parsed so far.
    Colon,
    /// The `{identifier} :` has been parsed so far.
    Type,
    /// The `{identifier} : {type}` has been parsed so far.
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        Self::Identifier
    }
}

///
/// The `field` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: FieldStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'field' statement.
    ///
    /// 'data: u64;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(FieldStatementBuilder, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::Identifier => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder.set_location(location);
                            let identifier = Identifier::new(location, identifier.inner);
                            self.builder.set_identifier(identifier);
                            self.state = State::Colon;
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
                State::Colon => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
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
                        } => Ok((self.builder, None)),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of_or_operator(
                                location,
                                vec![";"],
                                lexeme,
                                None,
                            ),
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
    use crate::tree::statement::field::Statement as FieldStatement;

    #[test]
    fn ok() {
        let input = r#"data: u64;"#;

        let expected = Ok((
            FieldStatement::new(
                Location::test(1, 1),
                false,
                Identifier::new(Location::test(1, 1), "data".to_owned()),
                Type::new(Location::test(1, 7), TypeVariant::integer_unsigned(64)),
            ),
            None,
        ));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"data;"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_type(
            Location::test(1, 5),
            Lexeme::Symbol(Symbol::Semicolon),
            Some(super::HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"a: u64"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 7),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default()
            .parse(TokenStream::test(input).wrap(), None)
            .map(|(builder, next)| (builder.finish(), next));

        assert_eq!(result, expected);
    }
}
