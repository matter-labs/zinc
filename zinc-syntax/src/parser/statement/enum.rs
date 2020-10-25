//!
//! The `enum` statement parser.
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
use crate::parser::variant_list::Parser as VariantListParser;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#enum::builder::Builder as EnumStatementBuilder;
use crate::tree::statement::r#enum::Statement as EnumStatement;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str =
    "enumeration type must have an identifier, e.g. `enum List { ... }`";

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    KeywordEnum,
    /// The `enum` has been parsed so far.
    Identifier,
    /// The `enum {identifier}` has been parsed so far.
    BracketCurlyLeftOrEnd,
    /// The `enum {identifier} {` has been parsed so far.
    VariantList,
    /// The `enum {identifier} { {variants}` has been parsed so far.
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        Self::KeywordEnum
    }
}

///
/// The `enum` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: EnumStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an 'enum' statement.
    ///
    /// '
    /// enum List {
    ///     A = 1,
    ///     B = 2,
    ///     C = 3,
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(EnumStatement, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::KeywordEnum => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Enum),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["enum"],
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
                        } => self.state = State::VariantList,
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::VariantList => {
                    let (variants, next) =
                        VariantListParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_variants(variants);
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
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::statement::r#enum::Statement as EnumStatement;
    use crate::tree::variant::Variant;

    #[test]
    fn ok_empty_with_brackets() {
        let input = r#"
    enum Test {}
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
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
    enum Test;
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![],
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::test(2, 14),
            )),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"
    enum Test {
        A = 1,
    }
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![Variant::new(
                    Location::test(3, 9),
                    Identifier::new(Location::test(3, 9), "A".to_owned()),
                    IntegerLiteral::new(
                        Location::test(3, 13),
                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                    ),
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
    enum Test {
        A = 1,
        B = 2,
        C = 3,
    }
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::test(2, 5),
                Identifier::new(Location::test(2, 10), "Test".to_owned()),
                vec![
                    Variant::new(
                        Location::test(3, 9),
                        Identifier::new(Location::test(3, 9), "A".to_owned()),
                        IntegerLiteral::new(
                            Location::test(3, 13),
                            LexicalIntegerLiteral::new_decimal("1".to_owned()),
                        ),
                    ),
                    Variant::new(
                        Location::test(4, 9),
                        Identifier::new(Location::test(4, 9), "B".to_owned()),
                        IntegerLiteral::new(
                            Location::test(4, 13),
                            LexicalIntegerLiteral::new_decimal("2".to_owned()),
                        ),
                    ),
                    Variant::new(
                        Location::test(5, 9),
                        Identifier::new(Location::test(5, 9), "C".to_owned()),
                        IntegerLiteral::new(
                            Location::test(5, 13),
                            LexicalIntegerLiteral::new_decimal("3".to_owned()),
                        ),
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
        let input = r#"enum { A = 1 };"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 6),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_right() {
        let input = r#"enum List { A = 1 );"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 19),
            vec!["}"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
