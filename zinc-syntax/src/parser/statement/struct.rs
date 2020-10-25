//!
//! The `struct` statement parser.
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
use crate::parser::field_list::Parser as FieldListParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#struct::builder::Builder as StructStatementBuilder;
use crate::tree::statement::r#struct::Statement as StructStatement;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str =
    "structure type must have an identifier, e.g. `struct Data { ... }`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordStruct,
    /// The `struct` has been parsed so far.
    Identifier,
    /// The `struct {identifier}` has been parsed so far.
    BracketCurlyLeftOrEnd,
    /// The `struct {identifier} {` has been parsed so far.
    FieldList,
    /// The `struct {identifier} { {fields}` has been parsed so far.
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordStruct
    }
}

///
/// The `struct` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: StructStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'struct' statement.
    ///
    /// '
    /// struct Data {
    ///     a: u8,
    ///     b: field,
    ///     c: bool,
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(StructStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordStruct => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["struct"],
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
                            self.state = State::BracketCurlyLeftOrEnd;
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
                State::BracketCurlyLeftOrEnd => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            ..
                        } => {
                            self.state = State::FieldList;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::FieldList => {
                    let (fields, next) =
                        FieldListParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_fields(fields);
                    self.next = next;
                    self.state = State::BracketCurlyRight;
                }
                State::BracketCurlyRight => {
                    return match crate::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of(location, vec!["}"], lexeme, None),
                        )),
                    };
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
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::field::Field;
    use crate::tree::identifier::Identifier;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;
    use crate::tree::statement::r#struct::Statement as StructStatement;

    #[test]
    fn ok_empty_with_brackets() {
        let input = r#"
    struct Test {}
"#;

        let expected = Ok((
            StructStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 12), "Test".to_owned()),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_empty_with_semicolon() {
        let input = r#"
    struct Test;
"#;

        let expected = Ok((
            StructStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 12), "Test".to_owned()),
                vec![],
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::test(2, 16),
            )),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"
    struct Test {
        a: u232,
    }
"#;

        let expected = Ok((
            StructStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 12), "Test".to_owned()),
                vec![Field::new(
                    Location::test(3, 9),
                    Identifier::new(Location::test(3, 9), "a".to_owned()),
                    Type::new(Location::test(3, 12), TypeVariant::integer_unsigned(232)),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"
    struct Test {
        a: u232,
        b: u232,
        c: u232,
    }
"#;

        let expected = Ok((
            StructStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 12), "Test".to_owned()),
                vec![
                    Field::new(
                        Location::test(3, 9),
                        Identifier::new(Location::test(3, 9), "a".to_owned()),
                        Type::new(Location::test(3, 12), TypeVariant::integer_unsigned(232)),
                    ),
                    Field::new(
                        Location::test(4, 9),
                        Identifier::new(Location::test(4, 9), "b".to_owned()),
                        Type::new(Location::test(4, 12), TypeVariant::integer_unsigned(232)),
                    ),
                    Field::new(
                        Location::test(5, 9),
                        Identifier::new(Location::test(5, 9), "c".to_owned()),
                        Type::new(Location::test(5, 12), TypeVariant::integer_unsigned(232)),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"struct { a: u8 };"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 8),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_right() {
        let input = r#"struct Data { a: u8 );"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 21),
            vec!["}"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
