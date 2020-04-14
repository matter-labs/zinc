//!
//! The enum statement parser.
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
use crate::syntax::parser::variant_list::Parser as VariantListParser;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::r#enum::builder::Builder as EnumStatementBuilder;
use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;

static HINT_EXPECTED_IDENTIFIER: &str =
    "enumeration type must have an identifier, e.g. `enum List { ... }`";

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordEnum,
    Identifier,
    BracketCurlyLeftOrEnd,
    VariantList,
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordEnum
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: EnumStatementBuilder,
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
        mut initial: Option<Token>,
    ) -> Result<(EnumStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordEnum => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Enum),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["enum"],
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
                        } => self.state = State::VariantList,
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::VariantList => {
                    let (variants, next) =
                        VariantListParser::default().parse(stream.clone(), None)?;
                    self.builder.set_variants(variants);
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
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;
    use crate::syntax::tree::variant::Variant;

    #[test]
    fn ok_empty_with_brackets() {
        let input = r#"
    enum Test {}
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
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
    enum Test;
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![],
            ),
            Some(Token::new(
                Lexeme::Symbol(Symbol::Semicolon),
                Location::new(2, 14),
            )),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

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
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![Variant::new(
                    Location::new(3, 9),
                    Identifier::new(Location::new(3, 9), "A".to_owned()),
                    IntegerLiteral::new(
                        Location::new(3, 13),
                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                    ),
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
    enum Test {
        A = 1,
        B = 2,
        C = 3,
    }
"#;

        let expected = Ok((
            EnumStatement::new(
                Location::new(2, 5),
                Identifier::new(Location::new(2, 10), "Test".to_owned()),
                vec![
                    Variant::new(
                        Location::new(3, 9),
                        Identifier::new(Location::new(3, 9), "A".to_owned()),
                        IntegerLiteral::new(
                            Location::new(3, 13),
                            LexicalIntegerLiteral::new_decimal("1".to_owned()),
                        ),
                    ),
                    Variant::new(
                        Location::new(4, 9),
                        Identifier::new(Location::new(4, 9), "B".to_owned()),
                        IntegerLiteral::new(
                            Location::new(4, 13),
                            LexicalIntegerLiteral::new_decimal("2".to_owned()),
                        ),
                    ),
                    Variant::new(
                        Location::new(5, 9),
                        Identifier::new(Location::new(5, 9), "C".to_owned()),
                        IntegerLiteral::new(
                            Location::new(5, 13),
                            LexicalIntegerLiteral::new_decimal("3".to_owned()),
                        ),
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
        let input = r#"enum { A = 1 };"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_identifier(
            Location::new(1, 6),
            Lexeme::Symbol(Symbol::BracketCurlyLeft),
            Some(super::HINT_EXPECTED_IDENTIFIER),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_curly_right() {
        let input = r#"enum List { A = 1 );"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 19),
            vec!["}"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
