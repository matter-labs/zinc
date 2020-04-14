//!
//! The struct statement parser.
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
use crate::syntax::parser::field_list::Parser as FieldListParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#struct::builder::Builder as StructStatementBuilder;
use crate::syntax::tree::statement::r#struct::Statement as StructStatement;

static HINT_EXPECTED_IDENTIFIER: &str =
    "structure type must have an identifier, e.g. `struct Data { ... }`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordStruct,
    Identifier,
    BracketCurlyLeftOrEnd,
    FieldList,
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordStruct
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: StructStatementBuilder,
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
        mut initial: Option<Token>,
    ) -> Result<(StructStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordStruct => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Struct),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["struct"],
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
                            self.state = State::BracketCurlyLeftOrEnd;
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
                State::BracketCurlyLeftOrEnd => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
                    let (fields, next) = FieldListParser::default().parse(stream.clone(), None)?;
                    self.builder.set_fields(fields);
                    self.next = next;
                    self.state = State::BracketCurlyRight;
                }
                State::BracketCurlyRight => {
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(Error::Syntax(
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
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::field::Field;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;
    use crate::syntax::tree::statement::r#struct::Statement as StructStatement;

    #[test]
    fn ok_empty_with_brackets() {
        let input = r#"
    struct Test {}
"#;

        let expected = Ok((
            StructStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 12), "Test".to_owned()),
                vec![],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_empty_with_semicolon() {
        let input = r#"
    struct Test;
"#;

        let expected = Ok((
            StructStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 12), "Test".to_owned()),
                vec![],
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::new(2, 16),
            )),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

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
                Location::new(2, 5),
                Identifier::new(Location::new(2, 12), "Test".to_owned()),
                vec![Field::new(
                    Location::new(3, 9),
                    Identifier::new(Location::new(3, 9), "a".to_owned()),
                    Type::new(Location::new(3, 12), TypeVariant::integer_unsigned(232)),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

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
                Location::new(2, 5),
                Identifier::new(Location::new(2, 12), "Test".to_owned()),
                vec![
                    Field::new(
                        Location::new(3, 9),
                        Identifier::new(Location::new(3, 9), "a".to_owned()),
                        Type::new(Location::new(3, 12), TypeVariant::integer_unsigned(232)),
                    ),
                    Field::new(
                        Location::new(4, 9),
                        Identifier::new(Location::new(4, 9), "b".to_owned()),
                        Type::new(Location::new(4, 12), TypeVariant::integer_unsigned(232)),
                    ),
                    Field::new(
                        Location::new(5, 9),
                        Identifier::new(Location::new(5, 9), "c".to_owned()),
                        Type::new(Location::new(5, 12), TypeVariant::integer_unsigned(232)),
                    ),
                ],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_identifier() {
        let input = r#"struct { a: u8 };"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 8),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_right() {
        let input = r#"struct Data { a: u8 );"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 21),
            vec!["}"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
