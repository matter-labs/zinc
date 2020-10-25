//!
//! The `mod` statement parser.
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
use crate::tree::identifier::Identifier;
use crate::tree::statement::module::builder::Builder as ModStatementBuilder;
use crate::tree::statement::module::Statement as ModStatement;

///
/// The `mod` statement parser.
///
#[derive(Default)]
pub struct Parser {
    /// The builder of the parsed value.
    builder: ModStatementBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a 'mod' statement.
    ///
    /// 'mod jabberwocky;'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ModStatement, Option<Token>), ParsingError> {
        self.next = initial;

        match crate::parser::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Mod),
                location,
            } => {
                self.builder.set_location(location);
            }
            Token { lexeme, location } => {
                return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                    location,
                    vec!["mod"],
                    lexeme,
                    None,
                )));
            }
        }

        match crate::parser::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                let identifier = Identifier::new(location, identifier.inner);
                self.builder.set_identifier(identifier);
            }
            Token { lexeme, location } => {
                return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                    location, lexeme, None,
                )))
            }
        }

        match crate::parser::take_or_next(self.next.take(), stream)? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => Ok((self.builder.finish(), None)),
            Token { lexeme, location } => Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                location,
                vec![";"],
                lexeme,
                None,
            ))),
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
    use crate::tree::statement::module::Statement as ModStatement;

    #[test]
    fn ok() {
        let input = r#"mod jabberwocky;"#;

        let expected = Ok((
            ModStatement::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 5), "jabberwocky".to_owned()),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_identifier() {
        let input = r#"mod;"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_identifier(
            Location::test(1, 4),
            Lexeme::Symbol(Symbol::Semicolon),
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_semicolon() {
        let input = r#"mod jabberwocky"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_one_of(
            Location::test(1, 16),
            vec![";"],
            Lexeme::Eof,
            None,
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
